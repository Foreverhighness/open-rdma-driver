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
/// trait BlueRdmaDevice<HAL: BlueHal, NIC: UdpAgent> {}
pub(super) trait BlueRdmaDevice<NIC: NicDevice> {}

mod example {
    use super::*;

    /// An blue RDMA device instance.
    pub(super) struct BlueRdmaDeviceInstance;
    /// Assume this instance can support `NicDevice` functionality.
    impl NicDevice for BlueRdmaDeviceInstance {}
    /// This instance rely on itself's `NicDevice` functionality.
    impl BlueRdmaDevice<Self> for BlueRdmaDeviceInstance {}
}

/// Telemetry for further research
pub(super) struct BlueRdmaDeviceTelemetry<NIC: NicDevice, DEV: BlueRdmaDevice<NIC>> {
    _nic: PhantomData<NIC>,
    _rdma_device: PhantomData<DEV>,
}
