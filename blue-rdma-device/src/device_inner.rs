//! Blue Rdma Emulator implementation

use core::net::Ipv4Addr;
use core::sync::atomic::AtomicBool;
use std::sync::Arc;

use eui48::MacAddress;
use flume::{Receiver, Sender};

use super::address::DmaAddress;
use super::csr::{EmulatorCsrs, EmulatorCsrsHandler};
use super::device_api::{ControlStatusRegisters, RawDevice};
use super::mr_table::{self, MemoryRegionTable};
use super::{dma, memory_region, net, queue_pair};
use crate::address::VirtualAddress;
use crate::dma::PointerMut;
use crate::third_party::net::{Metadata, PacketProcessor, RdmaMessage};

#[derive(Debug)]
enum State {
    NotReady,
}

#[derive(Debug)]
pub struct NetParameter {
    pub ip: Ipv4Addr,
    #[expect(unused, reason = "may use later")]
    pub gateway: Ipv4Addr,
    pub subnet_mask: Ipv4Addr,
    pub mac: MacAddress,
}

impl NetParameter {
    pub(crate) const fn new(ip: Ipv4Addr, gateway: Ipv4Addr, mask: Ipv4Addr, mac: MacAddress) -> Self {
        Self {
            ip,
            gateway,
            subnet_mask: mask,
            mac,
        }
    }
}

#[derive(Debug)]
pub struct DeviceInner<UA, DC, MRT = memory_region::Table>
where
    UA: net::Agent,
    DC: dma::Client,
    MRT: MemoryRegionTable,
{
    /// Control and Status Registers
    pub(crate) csrs: EmulatorCsrs,

    /// Udp agent
    pub(crate) udp_agent: std::sync::OnceLock<UA>,
    pub(crate) net_parameter: std::sync::OnceLock<Sender<NetParameter>>,

    /// DMA Client
    pub(crate) dma_client: DC,

    /// Thread Stop signal, may move out of this structure if I change this structure into `EmulatorInner`
    pub(crate) stop: AtomicBool,

    /// Emulator State
    #[expect(unused, reason = "may use later")]
    state: State,

    /// Memory Region Table (Key -> Context)
    pub(crate) mr_table: MRT,

    /// Page Table (index -> Vec<DmaAddress>)
    pub(crate) page_table: papaya::HashMap<u32, Vec<DmaAddress>>,

    /// Queue Pair Table (QPN -> Context)
    pub(crate) qp_table: queue_pair::Table,

    pub(crate) tx_command_request: Sender<()>,
    pub(crate) rx_command_request: Receiver<()>,

    pub(crate) tx_send: Sender<()>,
    pub(crate) rx_send: Receiver<()>,
}

impl<UA: net::Agent, DC: dma::Client, MRT: MemoryRegionTable> DeviceInner<UA, DC, MRT> {
    pub fn new(dma_client: DC, mr_table: MRT) -> Self {
        let (tx_command_request, rx_command_request) = flume::unbounded();
        let (tx_send, rx_send) = flume::unbounded();
        Self {
            udp_agent: Default::default(),
            net_parameter: Default::default(),
            dma_client,
            mr_table,
            csrs: EmulatorCsrs::default(),
            state: State::NotReady,
            stop: AtomicBool::default(),
            qp_table: Default::default(),
            tx_command_request,
            rx_command_request,
            tx_send,
            rx_send,
            page_table: papaya::HashMap::new(),
        }
    }

    pub(crate) const fn memory_region_table(&self) -> &MRT {
        &self.mr_table
    }

    pub(crate) const fn queue_pair_table(&self) -> &queue_pair::Table {
        &self.qp_table
    }
}

impl<UA, DC> DeviceInner<UA, DC>
where
    UA: net::Agent + Send + Sync + 'static,
    DC: dma::Client + Send + Sync + 'static,
{
    pub(super) fn start_net<F>(self: &Arc<Self>, f: F)
    where
        F: FnOnce(NetParameter) -> UA + Send + 'static,
    {
        let dev = Arc::clone(self);
        let (tx_net_para, rx_net_para) = flume::bounded(1);
        let _ = self.net_parameter.get_or_init(move || tx_net_para);

        let (tx, rx) = flume::unbounded();
        // TODO(fh): Store this handler properly
        let _handler_recv = std::thread::spawn(move || {
            let Ok(para) = rx_net_para.recv() else {
                return;
            };
            log::info!("network started with para: {para:?}");
            let udp_agent = f(para);
            let _ = dev.udp_agent.get_or_init(move || udp_agent);

            while !dev.stop.load(core::sync::atomic::Ordering::Relaxed) {
                // TODO(fh): Alloc buffer from MemoryPool.
                let mut buf = vec![0u8; 8192];
                let (len, src) = dev.udp_agent.get().unwrap().recv_from(&mut buf).expect("recv error");

                let ok = tx.send((buf, len, src)).is_ok();
                assert!(ok);
            }
        });

        let dev = Arc::clone(self);
        let _handler_packet = std::thread::spawn(move || {
            while let Ok((buf, len, src)) = rx.recv() {
                let msg = PacketProcessor::to_rdma_message(&buf[..len]).unwrap();
                log::debug!("receive data {msg:?} from {src:?}");

                dev.handle_message(&msg, src)
                    .unwrap_or_else(|_| panic!("handle message error: {msg:?}"));
            }
        });
    }

    pub fn start_work_queue(self: &Arc<Self>) {
        let dev = Arc::clone(self);
        // TODO(fh): Store this handler properly
        let _handler_command_request = std::thread::spawn(move || {
            // let _ = dev.queues_are_initialized.wait();
            dev.command_request_queue().run();
        });
        let dev = Arc::clone(self);
        let _handler_send = std::thread::spawn(move || {
            // let _ = dev.queues_are_initialized.wait();
            dev.send_queue().run();
        });
    }
}

impl<UA: net::Agent, DC: dma::Client, MRT: MemoryRegionTable> Drop for DeviceInner<UA, DC, MRT> {
    fn drop(&mut self) {
        self.stop.store(true, core::sync::atomic::Ordering::Relaxed);
    }
}

impl<UA: net::Agent, DC: dma::Client> RawDevice for DeviceInner<UA, DC> {
    fn csrs(&self) -> impl ControlStatusRegisters {
        EmulatorCsrsHandler::new(&self.csrs, self)
    }
}

impl<UA: net::Agent, DC: dma::Client> DeviceInner<UA, DC> {
    // TODO(fh): refactor to `copy_to_with_key(&self, src: &[u8], dst: &mut [u8], key: Key) -> super::Result`
    // or `copy_to_with_key(&self, src: &[u8], dst: ScatterGatherElement) -> super::Result`
    pub(crate) fn copy_to_with_key(&self, msg: &RdmaMessage) -> Result<(), mr_table::Error> {
        let data = &msg.payload.sg_list;
        assert_eq!(data.len(), 1, "currently only consider one Sge");
        let data = data[0];

        let Metadata::General(ref header) = msg.meta_data else {
            panic!("currently only consider write first and write last packet");
        };
        let key = header.reth.rkey.get().into();
        let va = VirtualAddress(header.reth.va);
        let access_flag = header.needed_permissions();

        let dma_addr = self
            .memory_region_table()
            .query(key, va, access_flag, &self.page_table)?;

        let ptr = self.dma_client.with_dma_addr::<u8>(dma_addr);
        unsafe { ptr.copy_from_nonoverlapping(data.data, data.len) };
        Ok(())
    }
}
