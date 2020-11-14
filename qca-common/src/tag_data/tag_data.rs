use crate::{IJsonSerializable, IYamlSerializable};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum PolymorphValue {
    Bool(bool),
    Integer(i64),
    Real(f64),
}

impl IJsonSerializable for PolymorphValue {}
impl IYamlSerializable for PolymorphValue {}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum MappingEndianness {
    LittleEndian,
    BigEndian,
}

impl IJsonSerializable for MappingEndianness {}
impl IYamlSerializable for MappingEndianness {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct MappingScalingReal {
    pub offset: f64,
    pub factor: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct MappingScalingInteger {
    pub offset: i64,
    pub factor: i64,
}

impl IJsonSerializable for MappingScalingInteger {}
impl IYamlSerializable for MappingScalingInteger {}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Ord, PartialOrd, Clone, Copy)]
#[repr(u8)]
pub enum MappingBit {
    B0 = 0,
    B1 = 1,
    B2 = 2,
    B3 = 3,
    B4 = 4,
    B5 = 5,
    B6 = 6,
    B7 = 7,
    B8 = 8,
    B9 = 9,
    B10 = 10,
    B11 = 11,
    B12 = 12,
    B13 = 13,
    B14 = 14,
    B15 = 15,
}

impl IJsonSerializable for MappingBit {}
impl IYamlSerializable for MappingBit {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum MappingMode {
    Boolean(MappingBit),
    Integer(MappingEndianness, MappingScalingInteger),
    Real(MappingEndianness, MappingScalingReal),
}

impl IJsonSerializable for MappingMode {}
impl IYamlSerializable for MappingMode {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TagReadConfig {
    pub name: String,
    pub unit: String,
    pub source_channel_id: usize,
    pub source_slave_id: usize,
    pub source_scanline_id: i16,
    pub source_scanline_offset: i16,
    pub source_scanline_length: usize,
    pub mapping_mode: Option<MappingMode>,
}

impl IJsonSerializable for TagReadConfig {}
impl IYamlSerializable for TagReadConfig {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TagReadData {
    pub name: String,
    pub unit: String,
    pub value: PolymorphValue,
    pub last_updated: Option<DateTime<FixedOffset>>,
}
