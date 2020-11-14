use crate::tag_data::tag_data::TagReadConfig;
use crate::{IJsonSerializable, IYamlSerializable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceConfig {
    pub name: String,
    pub reading_tags: Vec<TagReadConfig>,
}

impl IJsonSerializable for DeviceConfig {}
impl IYamlSerializable for DeviceConfig {}
