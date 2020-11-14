use qca_common::mbscanners::channel::{
    MBBaud, MBChannel, MBChannelAddress, MBChannelRTUAddress, MBChannelTCPAddress, MBDataBits, MBFlowControl, MBParity,
    MBStopBits,
};
use qca_common::mbscanners::device::MBDevice;
use qca_common::mbscanners::scanline::{MBScanline, MBScanlineCode};
use qca_common::mbscanners::MBConfig;
use qca_common::{IJsonSerializable, IYamlSerializable};

fn main() {
    let mut mb_config = MBConfig { channels: Vec::new() };
    mb_config.channels.push(MBChannel {
        timeout_ms: 500,
        interframe_delay_ms: 10,
        address: MBChannelAddress::TCP(MBChannelTCPAddress {
            socket_address: "127.0.0.1:502".parse().unwrap(),
        }),
        devices: vec![
            MBDevice {
                slave_address: 0,
                scanlines: vec![
                    MBScanline {
                        function_code: MBScanlineCode::FC03,
                        interval_ms: 1000,
                        length: 5,
                        start_address: 100,
                    },
                    MBScanline {
                        function_code: MBScanlineCode::FC01,
                        interval_ms: 1000,
                        length: 1,
                        start_address: 1,
                    },
                ],
            },
            MBDevice {
                slave_address: 1,
                scanlines: vec![MBScanline {
                    function_code: MBScanlineCode::FC03,
                    interval_ms: 200,
                    length: 1,
                    start_address: 1,
                }],
            },
        ],
    });
    mb_config.channels.push(MBChannel {
        timeout_ms: 500,
        interframe_delay_ms: 10,
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
                interval_ms: 1000,
                length: 3,
                start_address: 434,
            }],
        }],
    });
    println!("{}", mb_config.to_json_pretty());
    println!("{}", mb_config.to_yaml());
}
