#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HardwareScenario {
    GpsNoSignal = 0x01
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HardwareReason {
    ComponentConnectivity = 0x01
}