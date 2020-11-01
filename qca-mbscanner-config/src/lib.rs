pub mod channel;
pub mod device;
pub mod scanline;

use crate::channel::MBChannel;
use qca_common::{IJsonSerializable, IYamlSerializable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MBConfig {
    pub channels: Vec<MBChannel>,
}

impl IJsonSerializable for MBConfig {}
impl IYamlSerializable for MBConfig {}
