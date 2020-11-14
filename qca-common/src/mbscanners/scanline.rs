use crate::{IJsonSerializable, IYamlSerializable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Ord, PartialOrd, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum MBScanlineCode {
    FC01 = 0x01, // ReadCoilStatus
    FC02 = 0x02, // ReadInputStatus
    FC03 = 0x03, // ReadHoldingRegisters
    FC04 = 0x04, // ReadInputRegisters
}

impl IJsonSerializable for MBScanlineCode {}
impl IYamlSerializable for MBScanlineCode {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MBScanline {
    pub function_code: MBScanlineCode,
    pub interval_ms: u32,
    pub start_address: u16,
    pub length: u16,
}

impl IJsonSerializable for MBScanline {}
impl IYamlSerializable for MBScanline {}
