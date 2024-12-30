pub mod command_request {
    use core::net::Ipv4Addr;

    use eui48::MacAddress;
    use num_enum::{IntoPrimitive, TryFromPrimitive};

    use super::super::rdma::{Key, MemAccessTypeFlag, Pmtu, Psn, QpType, Qpn};

    #[derive(Debug, TryFromPrimitive, IntoPrimitive, PartialEq)]
    #[repr(u8)]
    pub(crate) enum CtrlRbDescOpcode {
        UpdateMrTable = 0x00,
        UpdatePageTable = 0x01,
        QpManagement = 0x02,
        SetNetworkParam = 0x03,
        SetRawPacketReceiveMeta = 0x04,
        UpdateErrorPsnRecoverPoint = 0x05,
    }

    #[derive(Debug)]
    pub(crate) enum ToCardCtrlRbDesc {
        UpdateMrTable(ToCardCtrlRbDescUpdateMrTable),
        UpdatePageTable(ToCardCtrlRbDescUpdatePageTable),
        QpManagement(ToCardCtrlRbDescQpManagement),
        SetNetworkParam(ToCardCtrlRbDescSetNetworkParam),
        SetRawPacketReceiveMeta(ToCardCtrlRbDescSetRawPacketReceiveMeta),
        UpdateErrorPsnRecoverPoint(ToCardCtrlRbDescUpdateErrPsnRecoverPoint),
    }

    #[derive(Debug, Default)]
    pub(crate) struct ToCardCtrlRbDescCommon {
        pub(crate) op_id: u32, // user_data
    }

    #[derive(Debug)]
    pub(crate) struct ToCardCtrlRbDescUpdateMrTable {
        pub(crate) common: ToCardCtrlRbDescCommon,
        pub(crate) addr: u64,
        pub(crate) len: u32,
        pub(crate) key: Key,
        pub(crate) pd_hdl: u32,
        pub(crate) acc_flags: MemAccessTypeFlag,
        pub(crate) pgt_offset: u32,
    }

    #[derive(Debug)]
    pub(crate) struct ToCardCtrlRbDescUpdatePageTable {
        pub(crate) common: ToCardCtrlRbDescCommon,
        pub(crate) start_addr: u64,
        pub(crate) pgt_idx: u32,  // offset
        pub(crate) pgte_cnt: u32, // bytes
    }

    #[derive(Debug)]
    pub(crate) struct ToCardCtrlRbDescQpManagement {
        pub(crate) common: ToCardCtrlRbDescCommon,
        pub(crate) is_valid: bool,
        pub(crate) qpn: Qpn,
        pub(crate) pd_hdl: u32,
        pub(crate) qp_type: QpType,
        pub(crate) rq_acc_flags: MemAccessTypeFlag,
        pub(crate) pmtu: Pmtu,
        pub(crate) peer_qpn: Qpn,
    }

    #[derive(Debug)]
    pub(crate) struct ToCardCtrlRbDescSetNetworkParam {
        pub(crate) common: ToCardCtrlRbDescCommon,
        pub(crate) gateway: Ipv4Addr,
        pub(crate) netmask: Ipv4Addr,
        pub(crate) ipaddr: Ipv4Addr,
        pub(crate) macaddr: MacAddress,
    }

    #[derive(Debug)]
    pub(crate) struct ToCardCtrlRbDescSetRawPacketReceiveMeta {
        pub(crate) common: ToCardCtrlRbDescCommon,
        pub(crate) base_write_addr: u64,
        pub(crate) key: Key,
    }

    #[derive(Debug)]
    pub(crate) struct ToCardCtrlRbDescUpdateErrPsnRecoverPoint {
        pub(crate) common: ToCardCtrlRbDescCommon,
        pub(crate) qpn: Qpn,
        pub(crate) recover_psn: Psn,
    }

    impl ToCardCtrlRbDesc {
        pub(crate) fn set_id(&mut self, id: u32) {
            match self {
                ToCardCtrlRbDesc::UpdateMrTable(desc) => desc.common.op_id = id,
                ToCardCtrlRbDesc::UpdatePageTable(desc) => desc.common.op_id = id,
                ToCardCtrlRbDesc::QpManagement(desc) => desc.common.op_id = id,
                ToCardCtrlRbDesc::SetNetworkParam(desc) => desc.common.op_id = id,
                ToCardCtrlRbDesc::SetRawPacketReceiveMeta(desc) => desc.common.op_id = id,
                ToCardCtrlRbDesc::UpdateErrorPsnRecoverPoint(desc) => desc.common.op_id = id,
            }
        }
    }

    pub mod descriptor {
        use bitfield::{BitRange, BitRangeMut, bitfield};

        bitfield! {
            pub struct CmdQueueDescCommonHead([u8]);
            u32;
            pub get_valid , set_valid: 0;
            pub get_is_success_or_need_signal_cplt, set_is_success_or_need_signal_cplt: 1;
            pub get_op_code, set_op_code: 7, 2;
            pub get_extra_segment_cnt, set_extra_segment_cnt: 11, 8;
            pub _reserverd, set_reserverd: 31, 12;
            pub get_user_data, set_user_data: 63, 32;
        }

        bitfield! {
            pub struct CmdQueueReqDescUpdateMrTable([u8]);
            u64;
            _cmd_queue_desc_common_head,_: 63, 0;      // 64bits
            pub get_mr_base_va, set_mr_base_va: 127, 64;   // 64bits
            pub get_mr_length, set_mr_length: 159, 128;    // 32bits
            pub get_mr_key, set_mr_key: 191, 160;          // 32bits
            pub get_pd_handler, set_pd_handler: 223, 192;  // 32bits
            pub get_acc_flags, set_acc_flags: 231, 224;    // 8bits
            pub get_pgt_offset, set_pgt_offset: 248, 232;  // 17bits
            _reserved0, _: 255, 249;                   // 7bits
        }

        bitfield! {
            pub struct CmdQueueReqDescUpdatePGT([u8]);
            u64;
            __cmd_queue_desc_common_head,_ : 63, 0;             // 64bits
            pub get_dma_addr, set_dma_addr: 127, 64;                // 64bits
            pub get_start_index, set_start_index: 159, 128;         // 32bits
            pub get_dma_read_length, set_dma_read_length: 191, 160; // 32bits
            _reserved0, _: 255, 192;                            // 64bits
        }

        bitfield! {
            pub struct CmdQueueReqDescQpManagementSeg0([u8]);
            u64;
            _cmd_queue_desc_common_head,_: 63, 0;                                       // 64bits
            pub get_is_valid, set_is_valid: 64;                                             // 1bit
            pub get_is_error, set_is_error: 65;                                             // 1bit
            _reserverd4, _: 71, 66;                                                     // 6bits
            pub get_qpn, set_qpn: 95, 72;                                                   // 24bits
            pub get_pd_handler, set_pd_handler: 127, 96;                                    // 32bits
            pub get_qp_type, set_qp_type: 131, 128;                                         // 4bits
            _reserverd3, _: 135, 132;                                                   // 4bits
            pub get_rq_access_flags, set_rq_access_flags: 143, 136;                         // 8bits
            pub get_pmtu, set_pmtu: 146, 144;                                               // 3bits
            _reserverd2, _: 151, 147;                                                   // 5bits
            pub get_peer_qpn, set_peer_qpn: 175, 152;                                      // 24bits
            _reserverd1, _: 255, 176;                                                   // 80bits
        }

        bitfield! {
            pub struct CmdQueueReqDescSetNetworkParam([u8]);
            u64;
            _cmd_queue_desc_common_head,_:          63 ,   0;                                       // 64bits
            pub get_ip_gateway, set_ip_gateway:         95 ,  64;                                       // 32bits
            pub get_ip_netmask, set_ip_netmask:         127,  96;                                       // 32bit
            pub get_ip_addr, set_ip_addr:               159, 128;                                       // 32bit
            _reserverd1, _:                         191, 160;                                       // 32bit
            pub get_eth_mac_addr, set_eth_mac_addr:     239, 192;                                       // 48bit
            _reserverd2, _:                         255, 240;                                       // 16bit
        }

        bitfield! {
            pub struct CmdQueueReqDescSetRawPacketReceiveMeta([u8]);
            u64;
            _cmd_queue_desc_common_head,_:              63 ,   0;                                   // 64bits
            pub get_write_base_addr, set_write_base_addr:   127,  64;                                   // 64bits
            pub get_write_mr_key, set_write_mr_key:         159, 128;                                   // 32bits
            _reserverd1, _:                             191, 160;                                   // 32bits
            _reserverd2, _:                             255, 240;                                   // 64bits
        }

        // typedef struct {
        //     ReservedZero#(136)              reserved1;      // 136 bits
        //     QPN                             qpn;            // 24  bits
        //     ReservedZero#(8)                reserved2;      // 8   bits
        //     PSN                             recoverPoint;   // 24  bits
        //     CmdQueueDescCommonHead          commonHeader;   // 64  bits
        // } CmdQueueReqDescUpdateErrorPsnRecoverPoint deriving(Bits, FShow);
        bitfield! {
            pub struct CmdQueueReqDescUpdateErrRecoverPoint([u8]);
            u32;
            _cmd_queue_desc_common_head,_:              63 ,   0;  // 64bits
            pub get_psn, set_psn:                       87 ,  64;  // 24bits
            _reserverd1, _:                             95 ,  88;  // 8 bits
            pub get_qpn, set_qpn:                       119,  96;  // 24bits
            _reserverd2, _:                             255, 120;  // 64bits
        }
    }
}

pub mod send {
    use core::net::Ipv4Addr;

    use eui48::MacAddress;
    use num_enum::{IntoPrimitive, TryFromPrimitive};

    use crate::third_party::rdma::{Key, Msn, Pmtu, Psn, QpType, Qpn, Sge, WorkReqSendFlag};

    #[derive(Debug, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
    #[repr(u8)]
    pub(crate) enum ToCardWorkRbDescOpcode {
        // IBV_WR_RDMA_WRITE           =  0,
        // IBV_WR_RDMA_WRITE_WITH_IMM  =  1,
        // IBV_WR_SEND                 =  2,
        // IBV_WR_SEND_WITH_IMM        =  3,
        // IBV_WR_RDMA_READ            =  4,
        // IBV_WR_ATOMIC_CMP_AND_SWP   =  5,
        // IBV_WR_ATOMIC_FETCH_AND_ADD =  6,
        // IBV_WR_LOCAL_INV            =  7,
        // IBV_WR_BIND_MW              =  8,
        // IBV_WR_SEND_WITH_INV        =  9,
        // IBV_WR_TSO                  = 10,
        // IBV_WR_DRIVER1              = 11,
        // IBV_WR_RDMA_READ_RESP       = 12, // Not defined in rdma-core
        // IBV_WR_FLUSH                = 14,
        // IBV_WR_ATOMIC_WRITE         = 15
        Write = 0,
        WriteWithImm = 1,
        Read = 4,
        ReadResp = 12, // Not defined in rdma-core
    }

    #[derive(Clone, Debug)]
    pub(crate) struct ToCardWorkRbDescCommon {
        pub(crate) total_len: u32,
        pub(crate) raddr: u64,
        pub(crate) rkey: Key,
        pub(crate) dqp_ip: Ipv4Addr,
        pub(crate) dqpn: Qpn,
        pub(crate) mac_addr: MacAddress,
        pub(crate) pmtu: Pmtu,
        pub(crate) flags: WorkReqSendFlag,
        pub(crate) qp_type: QpType,
        pub(crate) psn: Psn,
        pub(crate) msn: Msn,
    }

    impl Default for ToCardWorkRbDescCommon {
        fn default() -> Self {
            Self {
                total_len: 0,
                raddr: 0,
                rkey: Key::default(),
                dqp_ip: Ipv4Addr::new(0, 0, 0, 0),
                dqpn: Qpn::default(),
                mac_addr: MacAddress::default(),
                pmtu: Pmtu::Mtu256,
                flags: WorkReqSendFlag::empty(),
                qp_type: QpType::Rc,
                psn: Psn::default(),
                msn: Msn::default(),
            }
        }
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub(crate) struct DescSge {
        pub(crate) addr: u64,
        pub(crate) len: u32,
        pub(crate) key: Key,
    }

    impl From<Sge> for DescSge {
        fn from(sge: Sge) -> Self {
            Self {
                addr: sge.addr,
                len: sge.len,
                key: sge.key,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub(crate) enum ToCardWorkRbDesc {
        Read(ToCardWorkRbDescRead),
        Write(ToCardWorkRbDescWrite),
        WriteWithImm(ToCardWorkRbDescWriteWithImm),
        ReadResp(ToCardWorkRbDescWrite),
    }

    #[derive(Default, Clone, Debug)]
    pub(crate) struct ToCardWorkRbDescRead {
        pub(crate) common: ToCardWorkRbDescCommon,
        pub(crate) sge: DescSge,
    }

    #[derive(Default, Clone, Debug)]
    pub(crate) struct ToCardWorkRbDescWrite {
        pub(crate) common: ToCardWorkRbDescCommon,
        pub(crate) is_last: bool,
        pub(crate) is_first: bool,
        pub(crate) sge0: DescSge,
        pub(crate) sge1: Option<DescSge>,
        pub(crate) sge2: Option<DescSge>,
        pub(crate) sge3: Option<DescSge>,
    }

    #[derive(Clone, Debug, Default)]
    pub(crate) struct ToCardWorkRbDescWriteWithImm {
        pub(crate) common: ToCardWorkRbDescCommon,
        pub(crate) is_last: bool,
        pub(crate) is_first: bool,
        pub(crate) imm: u32,
        pub(crate) sge0: DescSge,
        pub(crate) sge1: Option<DescSge>,
        pub(crate) sge2: Option<DescSge>,
        pub(crate) sge3: Option<DescSge>,
    }

    pub mod descriptor {}
}

pub mod meta_report {
    use num_enum::{IntoPrimitive, TryFromPrimitive};

    #[derive(TryFromPrimitive, Clone, PartialEq, Eq, Debug, IntoPrimitive)]
    #[repr(u8)]
    pub(crate) enum ToHostWorkRbDescAethCode {
        // AETH_CODE_ACK  = 2'b00,
        // AETH_CODE_RNR  = 2'b01,
        // AETH_CODE_RSVD = 2'b10,
        // AETH_CODE_NAK  = 2'b11
        Ack = 0b00,
        Rnr = 0b01,
        Rsvd = 0b10,
        Nak = 0b11,
    }

    impl Default for ToHostWorkRbDescAethCode {
        fn default() -> Self {
            Self::Rsvd
        }
    }

    #[derive(TryFromPrimitive, Debug, Clone, IntoPrimitive)]
    #[repr(u8)]
    pub(crate) enum ToHostWorkRbDescStatus {
        Normal = 1,
        InvAccFlag = 2,
        InvOpcode = 3,
        InvMrKey = 4,
        InvMrRegion = 5,
        Unknown = 6,
    }

    impl Default for ToHostWorkRbDescStatus {
        fn default() -> Self {
            Self::Normal
        }
    }

    impl ToHostWorkRbDescStatus {
        pub(crate) fn is_ok(&self) -> bool {
            matches!(self, ToHostWorkRbDescStatus::Normal)
        }
    }

    #[derive(TryFromPrimitive, Debug, Clone, Copy, IntoPrimitive)]
    #[repr(u8)]
    pub(crate) enum ToHostWorkRbDescTransType {
        Rc = 0x00,
        Uc = 0x01,
        Rd = 0x02,
        Ud = 0x03,
        Cnp = 0x04,
        Xrc = 0x05,
        DtldExtended = 0x06, // Customize for normal packet.
    }

    impl Default for ToHostWorkRbDescTransType {
        fn default() -> Self {
            Self::Rc
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) enum ToHostWorkRbDescWriteType {
        First,
        Middle,
        Last,
        Only,
    }

    #[derive(TryFromPrimitive, PartialEq, Eq, Debug, Clone, IntoPrimitive)]
    #[repr(u8)]
    pub(crate) enum ToHostWorkRbDescOpcode {
        // SendFirst = 0x00,
        // SendMiddle = 0x01,
        // SendLast = 0x02,
        // SendLastWithImmediate = 0x03,
        // SendOnly = 0x04,
        // SendOnlyWithImmediate = 0x05,
        // RdmaWriteFirst = 0x06,
        // RdmaWriteMiddle = 0x07,
        // RdmaWriteLast = 0x08,
        // RdmaWriteLastWithImmediate = 0x09,
        // RdmaWriteOnly = 0x0a,
        // RdmaWriteOnlyWithImmediate = 0x0b,
        // RdmaReadRequest = 0x0c,
        // Acknowledge = 0x11,
        // AtomicAcknowledge = 0x12,
        // CompareSwap = 0x13,
        // FetchAdd = 0x14,
        // Resync = 0x15,
        // SendLastWithInvalidate = 0x16,
        // SendOnlyWithInvalidate = 0x17,
        RdmaWriteFirst = 0x06,
        RdmaWriteMiddle = 0x07,
        RdmaWriteLast = 0x08,
        RdmaWriteLastWithImmediate = 0x09,
        RdmaWriteOnly = 0x0a,
        RdmaWriteOnlyWithImmediate = 0x0b,
        RdmaReadResponseFirst = 0x0d,
        RdmaReadResponseMiddle = 0x0e,
        RdmaReadResponseLast = 0x0f,
        RdmaReadResponseOnly = 0x10,
        RdmaReadRequest = 0x0c,
        Acknowledge = 0x11,
    }

    impl ToHostWorkRbDescOpcode {
        pub(crate) fn is_first(&self) -> bool {
            match self {
                ToHostWorkRbDescOpcode::RdmaWriteFirst | ToHostWorkRbDescOpcode::RdmaReadResponseFirst => true,
                ToHostWorkRbDescOpcode::RdmaWriteMiddle
                | ToHostWorkRbDescOpcode::RdmaWriteLast
                | ToHostWorkRbDescOpcode::RdmaWriteLastWithImmediate
                | ToHostWorkRbDescOpcode::RdmaWriteOnly
                | ToHostWorkRbDescOpcode::RdmaWriteOnlyWithImmediate
                | ToHostWorkRbDescOpcode::RdmaReadResponseMiddle
                | ToHostWorkRbDescOpcode::RdmaReadResponseLast
                | ToHostWorkRbDescOpcode::RdmaReadResponseOnly
                | ToHostWorkRbDescOpcode::RdmaReadRequest
                | ToHostWorkRbDescOpcode::Acknowledge => false,
            }
        }

        pub(crate) fn is_resp(&self) -> bool {
            matches!(
                self,
                ToHostWorkRbDescOpcode::RdmaReadResponseFirst
                    | ToHostWorkRbDescOpcode::RdmaReadResponseMiddle
                    | ToHostWorkRbDescOpcode::RdmaReadResponseLast
                    | ToHostWorkRbDescOpcode::RdmaReadResponseOnly
            )
        }

        pub(crate) fn is_read_resp(&self) -> bool {
            matches!(
                self,
                ToHostWorkRbDescOpcode::RdmaReadResponseFirst
                    | ToHostWorkRbDescOpcode::RdmaReadResponseMiddle
                    | ToHostWorkRbDescOpcode::RdmaReadResponseLast
                    | ToHostWorkRbDescOpcode::RdmaReadResponseOnly
            )
        }

        pub(crate) fn write_type(&self) -> Option<ToHostWorkRbDescWriteType> {
            match self {
                ToHostWorkRbDescOpcode::RdmaWriteFirst | ToHostWorkRbDescOpcode::RdmaReadResponseFirst => {
                    Some(ToHostWorkRbDescWriteType::First)
                }
                ToHostWorkRbDescOpcode::RdmaWriteMiddle | ToHostWorkRbDescOpcode::RdmaReadResponseMiddle => {
                    Some(ToHostWorkRbDescWriteType::Middle)
                }
                ToHostWorkRbDescOpcode::RdmaWriteLast
                | ToHostWorkRbDescOpcode::RdmaWriteLastWithImmediate
                | ToHostWorkRbDescOpcode::RdmaReadResponseLast => Some(ToHostWorkRbDescWriteType::Last),
                ToHostWorkRbDescOpcode::RdmaWriteOnlyWithImmediate
                | ToHostWorkRbDescOpcode::RdmaWriteOnly
                | ToHostWorkRbDescOpcode::RdmaReadResponseOnly => Some(ToHostWorkRbDescWriteType::Only),
                ToHostWorkRbDescOpcode::RdmaReadRequest | ToHostWorkRbDescOpcode::Acknowledge => None,
            }
        }
    }

    pub mod descriptor {
        use bitfield::{BitRange, BitRangeMut, bitfield};

        bitfield! {
            pub struct MetaReportQueueDescFragAETH([u8]);
            no default BitRange;
            u32;
            pub get_psn, set_psn: 23, 0;          // 24bits
            pub get_msn, set_msn: 47, 24;         // 24bits
            pub get_aeth_value, set_aeth_value: 52, 48; // 5bits
            pub get_aeth_code, set_aeth_code: 54, 53;   // 2bits
            reserved1, _: 63, 55;                       // 9bits
        }

        impl<T: AsRef<[u8]>> BitRange<u32> for MetaReportQueueDescFragAETH<T> {
            #[allow(clippy::manual_bits, clippy::arithmetic_side_effects)]
            fn bit_range(&self, msb: usize, lsb: usize) -> u32 {
                let bit_len = size_of::<u8>() * 8;
                let value_bit_len = size_of::<u32>() * 8;
                let mut value = 0;
                for i in (lsb..=msb).rev() {
                    value <<= 1_i32;
                    let ptr = self.0.as_ref().as_ptr();
                    let v_ptr = unsafe { ptr.add(i / bit_len) };
                    let v = unsafe { core::ptr::read_volatile(v_ptr) };
                    value |= ((v >> (i % bit_len)) & 1) as u32;
                }
                value << (value_bit_len - (msb - lsb + 1)) >> (value_bit_len - (msb - lsb + 1))
            }
        }
        #[allow(clippy::manual_bits, clippy::arithmetic_side_effects)]
        impl<T: AsMut<[u8]>> BitRangeMut<u32> for MetaReportQueueDescFragAETH<T> {
            fn set_bit_range(&mut self, msb: usize, lsb: usize, value: u32) {
                let bit_len = size_of::<u8>() * 8;
                let mut value = value;
                for i in lsb..=msb {
                    self.0.as_mut()[i / bit_len] &= !(1 << (i % bit_len));
                    self.0.as_mut()[i / bit_len] |= ((value & 1) as u8) << (i % bit_len);
                    value >>= 1_i32;
                }
            }
        }
    }
}
