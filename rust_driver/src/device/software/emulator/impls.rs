//! Blue Rdma Emulator implementation

use core::net::Ipv4Addr;
use core::sync::atomic::AtomicBool;
use std::sync::Arc;

use eui48::MacAddress;
use flume::{Receiver, Sender};

use super::address::DmaAddress;
use super::csr::{EmulatorCsrs, EmulatorCsrsHandler};
use super::device_api::{ControlStatusRegisters, RawDevice};
use super::mr_table::MemoryRegionTable;
use super::{dma, memory_region, net, queue_pair, simulator};
use crate::device::software::packet_processor::PacketProcessor;

#[derive(Debug)]
enum State {
    NotReady,
}

#[derive(Debug)]
pub(crate) struct NetParameter {
    pub ip: Ipv4Addr,
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
pub struct Emulator<UA = simulator::UdpAgent, DC = simulator::DmaClient, MRT = memory_region::Table>
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

impl<UA: net::Agent, DC: dma::Client, MRT: MemoryRegionTable> Emulator<UA, DC, MRT> {
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

    pub(crate) fn memory_region_table(&self) -> &MRT {
        &self.mr_table
    }

    pub(crate) fn queue_pair_table(&self) -> &queue_pair::Table {
        &self.qp_table
    }
}

impl<UA> Emulator<UA>
where
    UA: net::Agent + Send + Sync + 'static,
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
                    .expect(&format!("handle message error: {msg:?}"));
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

impl<UA: net::Agent, DC: dma::Client, MRT: MemoryRegionTable> Drop for Emulator<UA, DC, MRT> {
    fn drop(&mut self) {
        self.stop.store(true, core::sync::atomic::Ordering::Relaxed);
    }
}

impl<UA: net::Agent> RawDevice for Emulator<UA> {
    fn csrs(&self) -> impl ControlStatusRegisters {
        EmulatorCsrsHandler::new(&self.csrs, self)
    }
}
