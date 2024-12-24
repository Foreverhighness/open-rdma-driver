mod capture;

use core::time::Duration;

use open_rdma_driver::device::software::emulator::simulator::csr_proxy::Proxy;
use open_rdma_driver::device::software::emulator::simulator::rpc::{Client, RpcClient};
use open_rdma_driver::device::software::emulator::DeviceInner;

fn main() {
    env_logger::init();

    let rpc = RpcClient;
    let client_id = unsafe {
        // std::env::set_var("MOCK_HOST_SERVER_ADDR", "0.0.0.0");
        // std::env::set_var("MOCK_HOST_SERVER_PORT", "9876");
        rpc.c_createBRAM(512, 1024 * 1024)
    };
    let dev = DeviceInner::new_simulator(client_id);
    let csr_proxy = Proxy::new(client_id, rpc, dev);
    let (read_thread, write_thread, stop) = csr_proxy.run();

    println!("running simulator");
    std::thread::sleep(Duration::from_secs(180));
    println!("exit simulator");

    stop.store(true, core::sync::atomic::Ordering::Relaxed);
    read_thread.join().unwrap();
    write_thread.join().unwrap();
}
