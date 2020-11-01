use crate::device::MBDevice;
use qca_common::{IJsonSerializable, IYamlSerializable};
use serde::{Deserialize, Serialize};
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::net::SocketAddrV4;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MBFlowControl {
    None,
    Software,
    Hardware,
}

impl Into<FlowControl> for MBFlowControl {
    fn into(self) -> FlowControl {
        match self {
            MBFlowControl::None => FlowControl::None,
            MBFlowControl::Software => FlowControl::Software,
            MBFlowControl::Hardware => FlowControl::Hardware,
        }
    }
}

impl IJsonSerializable for MBFlowControl {}
impl IYamlSerializable for MBFlowControl {}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MBParity {
    None,
    Odd,
    Even,
}

impl Into<Parity> for MBParity {
    fn into(self) -> Parity {
        match self {
            MBParity::None => Parity::None,
            MBParity::Odd => Parity::Odd,
            MBParity::Even => Parity::Even,
        }
    }
}

impl IJsonSerializable for MBParity {}
impl IYamlSerializable for MBParity {}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MBDataBits {
    Seven,
    Eight,
}

impl Into<DataBits> for MBDataBits {
    fn into(self) -> DataBits {
        match self {
            MBDataBits::Seven => DataBits::Seven,
            MBDataBits::Eight => DataBits::Eight,
        }
    }
}

impl IJsonSerializable for MBDataBits {}
impl IYamlSerializable for MBDataBits {}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MBStopBits {
    One,
    Two,
}

impl Into<StopBits> for MBStopBits {
    fn into(self) -> StopBits {
        match self {
            MBStopBits::One => StopBits::One,
            MBStopBits::Two => StopBits::Two,
        }
    }
}

impl IJsonSerializable for MBStopBits {}
impl IYamlSerializable for MBStopBits {}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum MBBaud {
    B9600 = 9600,
    B19200 = 19200,
    B38400 = 38400,
    B57600 = 57600,
    B115200 = 115200,
}

impl IJsonSerializable for MBBaud {}
impl IYamlSerializable for MBBaud {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MBChannelTCPAddress {
    pub socket_address: SocketAddrV4,
}

impl IJsonSerializable for MBChannelTCPAddress {}
impl IYamlSerializable for MBChannelTCPAddress {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MBChannelRTUAddress {
    pub port_address: String,
    pub baud: MBBaud,
    pub data_bits: MBDataBits,
    pub parity: MBParity,
    pub stop_bits: MBStopBits,
    pub flow_control: MBFlowControl,
}

impl IJsonSerializable for MBChannelRTUAddress {}
impl IYamlSerializable for MBChannelRTUAddress {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MBChannelAddress {
    TCP(MBChannelTCPAddress),
    RTU(MBChannelRTUAddress),
}

impl IJsonSerializable for MBChannelAddress {}
impl IYamlSerializable for MBChannelAddress {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MBChannel {
    pub address: MBChannelAddress,
    pub timeout: Duration,
    pub interframe_delay: Duration,
    pub devices: Vec<MBDevice>,
}

impl IJsonSerializable for MBChannel {}
impl IYamlSerializable for MBChannel {}
