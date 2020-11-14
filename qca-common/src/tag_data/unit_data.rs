use crate::tag_data::device_data::DeviceConfig;
use crate::{IJsonSerializable, IYamlSerializable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitConfig {
    pub name: String,
    pub devices: Vec<DeviceConfig>,
}

impl IJsonSerializable for UnitConfig {}
impl IYamlSerializable for UnitConfig {}
