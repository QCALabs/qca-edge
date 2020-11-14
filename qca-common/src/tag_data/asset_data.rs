use crate::tag_data::unit_data::UnitConfig;
use crate::{IJsonSerializable, IYamlSerializable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetConfig {
    pub name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub units: Vec<UnitConfig>,
}

impl IJsonSerializable for AssetConfig {}
impl IYamlSerializable for AssetConfig {}
