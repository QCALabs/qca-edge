use qca_common::{IJsonSerializable, IYamlSerializable};
use qca_mbscanner_config::channel::{
    MBBaud, MBChannel, MBChannelAddress, MBChannelRTUAddress, MBChannelTCPAddress, MBDataBits, MBFlowControl, MBParity,
    MBStopBits,
};
use qca_mbscanner_config::device::MBDevice;
use qca_mbscanner_config::scanline::{MBScanline, MBScanlineCode};
use qca_mbscanner_config::MBConfig;
use std::time::Duration;

fn main() {
    let mut mb_config = MBConfig { channels: Vec::new() };
    mb_config.channels.push(MBChannel {
        timeout: Duration::from_millis(500),
        interframe_delay: Duration::from_millis(10),
        address: MBChannelAddress::TCP(MBChannelTCPAddress {
            socket_address: "127.0.0.1:502".parse().unwrap(),
        }),
        devices: vec![
            MBDevice {
                slave_address: 0,
                scanlines: vec![
                    MBScanline {
                        function_code: MBScanlineCode::FC03,
                        interval: Duration::from_secs(1),
                        length: 5,
                        start_address: 100,
                    },
                    MBScanline {
                        function_code: MBScanlineCode::FC01,
                        interval: Duration::from_secs(1),
                        length: 1,
                        start_address: 1,
                    },
                ],
            },
            MBDevice {
                slave_address: 1,
                scanlines: vec![MBScanline {
                    function_code: MBScanlineCode::FC03,
                    interval: Duration::from_millis(200),
                    length: 1,
                    start_address: 1,
                }],
            },
        ],
    });
    mb_config.channels.push(MBChannel {
        timeout: Duration::from_millis(500),
        interframe_delay: Duration::from_millis(10),
        address: MBChannelAddress::RTU(MBChannelRTUAddress {
            port_address: "/dev/ttyUSB0".into(),
            baud: MBBaud::B115200,
            data_bits: MBDataBits::Eight,
            parity: MBParity::None,
            stop_bits: MBStopBits::Two,
            flow_control: MBFlowControl::None,
        }),
        devices: vec![MBDevice {
            slave_address: 1,
            scanlines: vec![MBScanline {
                function_code: MBScanlineCode::FC03,
                interval: Duration::from_secs(1),
                length: 3,
                start_address: 434,
            }],
        }],
    });
    println!("{}", mb_config.to_json_pretty());
    println!("{}", mb_config.to_yaml());
}
