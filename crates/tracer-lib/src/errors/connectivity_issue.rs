#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectivityScenario {
    LoraJoinTimeout = 0x01,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectivityReason {
    Timeout = 0x01,
}