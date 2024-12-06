use core::time::Duration;

mod rpc;

use rpc::*;

fn main() {
    let client_id = 1;
    let _mem_ptr = unsafe {
        std::env::set_var("MOCK_HOST_SERVER_ADDR", "0.0.0.0");
        std::env::set_var("MOCK_HOST_SERVER_PORT", "9876");
        c_createBRAM(512, 1024 * 1024)
    };

    let mut nic_payload = RpcNetIfcRxTxPayload::new();
    let nic_ptr = &raw mut nic_payload;
    // let mut bar_payload = BarIoInfo::new();
    // let bar_ptr = &raw mut bar_payload;
    // let mut data = 0;
    // let csr_addr = 0;
    // let word_width = 4;
    // let mut cnt = 0;
    for _ in 0..100 {
        unsafe {
            c_netIfcGetRxData(nic_ptr, client_id, 0);
            // c_netIfcPutTxData(client_id, nic_ptr);
            // c_getPcieBarReadReq(bar_ptr, client_id);
            // c_getPcieBarWriteReq(bar_ptr, client_id);
            // c_putPcieBarReadResp(client_id, bar_ptr);
            // c_putPcieBarWriteResp(client_id, bar_ptr);
            // c_readBRAM(&raw mut data, client_id, csr_addr, word_width);
            // c_writeBRAM(client_id, csr_addr, &raw mut data, &raw mut cnt, word_width);
        }
        std::thread::sleep(Duration::from_millis(100));
        if nic_payload.is_valid == 0 {
            continue;
        }
        println!("Get {:?}", nic_payload);
    }
    std::thread::sleep(Duration::from_secs(10));
}
