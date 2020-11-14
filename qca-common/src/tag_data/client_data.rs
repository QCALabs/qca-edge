use crate::tag_data::asset_data::AssetConfig;
use crate::{IJsonSerializable, IYamlSerializable};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientConfig {
    pub name: String,
    pub registered_at: Option<DateTime<FixedOffset>>,
    pub assets: Vec<AssetConfig>,
}

impl IJsonSerializable for ClientConfig {}
impl IYamlSerializable for ClientConfig {}
