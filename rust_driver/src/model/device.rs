//! Device conceptions
//!
//! A lot of code is inspired by <https://github.com/KuangjuX/ixgbe-driver/blob/main/src/lib.rs#L51>

use core::marker::PhantomData;

pub(super) trait NicDevice {}

/// Represents a blue RDMA device
///
/// # Why design this trait
///
/// I need to support both hardware and software, as well as emulators.
/// While the hardware is constantly iterating, I can design a different implementation for each hardware version,
/// but I don't want to change the implementation of the `Driver` layer.
///
/// # Goal
///
/// Hide enough hardware details while providing a simple and stable interface to the `Driver` layer.
///
/// I want hide
///     1. protocol detail: how packet assembled
///     2. DMA access: I want device layer to use `HardwareAddress`, `Driver` layer only see `CPUVirtualAddress`
///
/// # Design options
///
/// 1. Design the interface by using hardware knowledge.
///   I already know that there are 4 queues in the hardware, so I can design the interface according to the
/// functionality of each queue.
///
/// 2. Design the hierarchy by separate dependencies.
///   For example, only one of the `Driver` or `Device` layers should be directly dependent on `Scheduler` module, and
/// the same for the `DMAAllocator` module.
/// Is `NicDevice` need? for `RoCEv2` device, maybe we just need an `UdpAgent`.
/// Answer: `NicDevice` is not necessary, `NicDevice` could be an optional field in struct definition.
pub(super) trait BlueRdmaDevice {}

mod instance {
    use super::*;

    /// Hardware device
    struct Hardware;
    impl BlueRdmaDevice for Hardware {}

    /// Simulator device
    struct Simulator;
    impl BlueRdmaDevice for Simulator {}

    /// Emulated device
    struct Emulator<Nic: NicDevice> {
        _udp_agent: Nic,
    }
    /// Only emulator need nic support.
    impl BlueRdmaDevice for Emulator<UdpAgent> {}

    /// Support `NicDevice`
    struct UdpAgent;
    impl NicDevice for UdpAgent {}
}

/// Telemetry for further research
pub(super) struct BlueRdmaDeviceTelemetry<Dev: BlueRdmaDevice> {
    _rdma_device: PhantomData<Dev>,
}
