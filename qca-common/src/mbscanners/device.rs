use crate::mbscanners::scanline::MBScanline;
use crate::{IJsonSerializable, IYamlSerializable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MBDevice {
    pub slave_address: u8,
    pub scanlines: Vec<MBScanline>,
}

impl IJsonSerializable for MBDevice {}
impl IYamlSerializable for MBDevice {}
