use core::time::Duration;

mod net;
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
    let mut buffer = Vec::new();
    // let mut bar_payload = BarIoInfo::new();
    // let bar_ptr = &raw mut bar_payload;
    // let mut data = 0;
    // let csr_addr = 0;
    // let word_width = 4;
    let mut cnt = 0;
    let mut fragment = 0;
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
        std::thread::sleep(Duration::from_millis(10));
        if nic_payload.is_valid == 0 {
            continue;
        }

        // generate fragment files
        {
            let filename = &format!("fragment-{cnt}-{fragment}.bin");
            fragment += 1;

            let json = serde_json::to_vec(&nic_payload).unwrap();

            std::fs::write(filename, &json).unwrap();
            let read_back = std::fs::read(filename).unwrap();

            let value: RpcNetIfcRxTxPayload = serde_json::from_slice(&read_back).unwrap();
            assert_eq!(value, nic_payload);
        }

        let payload = unsafe { core::mem::transmute::<_, [u8; 64]>(nic_payload.data) };
        buffer.extend_from_slice(&payload);

        if nic_payload.is_last == 1 {
            // generate frame files
            {
                let filename = &format!("ethernet-frame-{cnt}.bin");
                cnt += 1;
                std::fs::write(filename, &buffer).unwrap();
                assert_eq!(std::fs::read(filename).unwrap(), buffer);
            }

            let eth_frame = smoltcp::wire::EthernetFrame::new_checked(buffer.as_slice()).unwrap();
            println!("{}", smoltcp::wire::pretty_print::PrettyPrinter::print(&eth_frame));

            buffer.clear();
        }
    }
    std::thread::sleep(Duration::from_secs(10));
}
