use super::{hardware_issue::HardwareScenario, hardware_issue::HardwareReason};
use super::{connectivity_issue::ConnectivityScenario, connectivity_issue::ConnectivityReason};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Issue {
    HardwareFault { scenario: HardwareScenario, reason: HardwareReason },
    ConnectivityFault { scenario: ConnectivityScenario, reason: ConnectivityReason },
}

impl Issue {
    pub fn to_bytes(&self) -> [u8; 3] {
        [self.category(), self.scenario(), self.reason()]
    }

    
    pub fn category(&self) -> u8 {
        match self {
            Issue::HardwareFault { .. } => 0x01,
            Issue::ConnectivityFault { .. } => 0x02
        }
    }

    pub fn scenario(&self) -> u8 {
        match self {
            Issue::HardwareFault { scenario, .. } => *scenario as u8,
            Issue::ConnectivityFault { scenario, .. } => *scenario as u8,
        }
    }

    pub fn reason(&self) -> u8 {
        match self {
            Issue::HardwareFault { reason, .. } => *reason as u8,
            Issue::ConnectivityFault { reason, .. } => *reason as u8,
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_hardware_fault_category_value() {
        assert_eq!(Issue::HardwareFault { scenario: HardwareScenario::GpsNoSignal, 
                                          reason: HardwareReason::ComponentConnectivity }.category(), 0x01);
    }

    #[test]
    fn check_connectivity_fault_category_value() {
        assert_eq!(Issue::ConnectivityFault { scenario: ConnectivityScenario::LoraJoinTimeout, 
                                              reason: ConnectivityReason::Timeout }.category(), 0x02);
    }

    #[test]
    fn check_hardware_fault_scenario_value() {
        assert_eq!(Issue::HardwareFault { scenario: HardwareScenario::GpsNoSignal, 
                                          reason: HardwareReason::ComponentConnectivity }.scenario(), 0x01);
    }

    #[test]
    fn check_connectivity_fault_scenario_value() {
        assert_eq!(Issue::ConnectivityFault { scenario: ConnectivityScenario::LoraJoinTimeout, 
                                              reason: ConnectivityReason::Timeout }.scenario(), 0x01);
    }

    #[test]
    fn check_hardware_fault_reason_value() {
        assert_eq!(Issue::HardwareFault { scenario: HardwareScenario::GpsNoSignal, 
                                          reason: HardwareReason::ComponentConnectivity }.reason(), 0x01);
    }

    #[test]
    fn check_connectivity_fault_reason_value() {
        assert_eq!(Issue::ConnectivityFault { scenario: ConnectivityScenario::LoraJoinTimeout, 
                                              reason: ConnectivityReason::Timeout }.reason(), 0x01);
    }

    #[test]
    fn check_hardware_byte_packing() {
        let issue = Issue::HardwareFault { scenario: HardwareScenario::GpsNoSignal, reason: HardwareReason::ComponentConnectivity };
        assert_eq!(issue.to_bytes(), [0x01, 0x01, 0x01]);
    }

    #[test]
    fn check_connectivity_byte_packing() {
        let issue = Issue::ConnectivityFault { scenario: ConnectivityScenario::LoraJoinTimeout, reason: ConnectivityReason::Timeout };
        assert_eq!(issue.to_bytes(), [0x02, 0x01, 0x01]);
    }
}