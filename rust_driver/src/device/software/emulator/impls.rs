//! Blue Rdma Emulator implementation

use core::sync::atomic::AtomicBool;
use std::sync::Arc;

use flume::{Receiver, Sender};

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
pub struct Emulator<UA = simulator::UdpAgent, DC = simulator::DmaClient, MRT = memory_region::Table>
where
    UA: net::Agent,
    DC: dma::Client,
    MRT: MemoryRegionTable,
{
    /// Control and Status Registers
    pub(crate) csrs: EmulatorCsrs,

    /// Udp agent
    udp_agent: UA,

    /// DMA Client
    pub(crate) dma_client: DC,

    /// Thread Stop signal, may move out of this structure if I change this structure into `EmulatorInner`
    pub(crate) stop: AtomicBool,

    /// Emulator State
    state: State,

    /// Memory Region Table (Key -> Context)
    mr_table: MRT,

    /// Queue Pair Table (QPN -> Context)
    qp_table: queue_pair::Table,

    pub(crate) tx_command_request: Sender<()>,
    pub(crate) rx_command_request: Receiver<()>,

    pub(crate) tx_send: Sender<()>,
    pub(crate) rx_send: Receiver<()>,
}

impl<UA: net::Agent, DC: dma::Client, MRT: MemoryRegionTable> Emulator<UA, DC, MRT> {
    pub fn new(udp_agent: UA, dma_client: DC, mr_table: MRT) -> Self {
        let (tx_command_request, rx_command_request) = flume::unbounded();
        let (tx_send, rx_send) = flume::unbounded();
        Self {
            udp_agent,
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
    pub(super) fn start_net(self: &Arc<Self>) {
        let dev = Arc::clone(self);

        // TODO(fh): Store this handler properly
        let _handle = std::thread::spawn(move || {
            // TODO(fh): Alloc buffer from MemoryPool.
            let mut buf = vec![0u8; 8192];
            while !dev.stop.load(core::sync::atomic::Ordering::Relaxed) {
                let (len, src) = dev.udp_agent.recv_from(&mut buf).expect("recv error");

                let msg = PacketProcessor::to_rdma_message(&buf[..len]).unwrap();
                log::debug!("receive data {msg:#?} from {src:?}");
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
