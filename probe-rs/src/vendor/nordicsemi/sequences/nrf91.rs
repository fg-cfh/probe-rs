//! Sequences for the nRF91.

use std::sync::Arc;

use super::nrf::Nrf;
use crate::architecture::arm::ap::AccessPort;
use crate::architecture::arm::memory::adi_v5_memory_interface::ArmProbe;
use crate::architecture::arm::sequences::ArmDebugSequence;
use crate::architecture::arm::ArmError;
use crate::architecture::arm::{
    communication_interface::Initialized, ArmCommunicationInterface, DapAccess,
    FullyQualifiedApAddress,
};

/// The sequence handle for the nRF9160.
#[derive(Debug)]
pub struct Nrf9160(());

impl Nrf9160 {
    /// Create a new sequence handle for the nRF9160.
    pub fn create() -> Arc<dyn ArmDebugSequence> {
        Arc::new(Self(()))
    }
}

impl Nrf for Nrf9160 {
    fn core_aps(
        &self,
        memory: &mut dyn ArmProbe,
    ) -> Vec<(FullyQualifiedApAddress, FullyQualifiedApAddress)> {
        let memory_ap = memory.ap();
        let ap_address = memory_ap.ap_address();

        let core_aps = [(0, 4)];

        core_aps
            .into_iter()
            .map(|(core_ahb_ap, core_ctrl_ap)| {
                (
                    FullyQualifiedApAddress::v1_with_dp(ap_address.dp(), core_ahb_ap),
                    FullyQualifiedApAddress::v1_with_dp(ap_address.dp(), core_ctrl_ap),
                )
            })
            .collect()
    }

    fn is_core_unlocked(
        &self,
        arm_interface: &mut ArmCommunicationInterface<Initialized>,
        _ahb_ap_address: &FullyQualifiedApAddress,
        ctrl_ap_address: &FullyQualifiedApAddress,
    ) -> Result<bool, ArmError> {
        let approtect_status = arm_interface.read_raw_ap_register(ctrl_ap_address, 0x00C)?;
        Ok(approtect_status != 0)
    }

    fn has_network_core(&self) -> bool {
        false
    }
}
