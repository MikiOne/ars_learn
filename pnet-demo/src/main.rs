//! - github: https://github.com/libpnet/libpnet
use pnet::{datalink::{self, Channel}, packet::ethernet::EthernetPacket};
use pnet::packet::{FromPacket, Packet};

fn simple() {
    // 选取 'en0' 网络接口
    let interface = datalink::interfaces().into_iter()
        .find(|interface| interface.name == "en0")
        .unwrap();

    // 抓包需要数据链路通道，它建立了到网络接口'en0' 的底层链路。
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    println!("开始读取网络数据包: ");

    // 建立一个循环来连续读取传入的数据包
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("新数据包:");
                    println!(
                        "{} => {}: {}",
                        ethernet_packet.get_destination(),
                        ethernet_packet.get_source(),
                        ethernet_packet.get_ethertype()
                    );
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

/// 包捕获通常需要以超级用户(根)权限运行：
/// ```
/// sudo cargo run
/// ```
/// 运行结果
/// ```
/// 开始读取网络数据包:
/// 新数据包:
/// ff:ff:ff:ff:ff:ff => 08:a1:89:99:76:65: Arp
/// 新数据包:
/// 58:47:ca:73:98:9d => 14:7d:da:9a:d2:93: Ipv4
/// 新数据包:
/// 58:47:ca:73:98:9d => 14:7d:da:9a:d2:93: Ipv4
/// 新数据包:
/// 01:00:5e:7f:ff:fa => 38:fc:98:a0:ae:4c: Ipv4
/// 新数据包:
/// ff:ff:ff:ff:ff:ff => 60:1d:9d:47:51:9e: Ipv4
/// 新数据包:
/// 01:00:5e:00:00:fb => 60:1d:9d:47:51:9e: Ipv4
/// 新数据包:
/// 01:00:5e:00:00:16 => e6:6a:89:26:e9:7a: Ipv4
/// 新数据包:
/// 01:00:5e:7f:ff:fa => 38:fc:98:a0:ae:4c: Ipv4
/// 新数据包:
/// ff:ff:ff:ff:ff:ff => 24:28:fd:90:56:24: Arp
/// 新数据包:
/// ff:ff:ff:ff:ff:ff => 08:a1:89:23:54:7c: Arp
/// 新数据包:
/// ff:ff:ff:ff:ff:ff => 08:a1:89:ba:d2:2d: Arp
/// 新数据包:
/// ff:ff:ff:ff:ff:ff => 08:a1:89:ba:d1:90: Arp
/// 新数据包:
/// 01:00:5e:7f:ff:fa => 60:1d:9d:47:51:9e: Ipv4
/// ```


fn simple_2() {
    // 选取 'en0' 网络接口
    let interface = datalink::interfaces().into_iter()
        .find(|interface| interface.name == "en0")
        .unwrap();

    // 抓包需要数据链路通道，它建立了到网络接口'en0' 的底层链路。
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    println!("开始读取网络数据包: ");
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("新数据包:");
                    // ......

                    let packet = ethernet_packet.packet();
                    let payload = ethernet_packet.payload();
                    let from_packet = ethernet_packet.from_packet();
                    println!("---");
                    println!("packet: {:?}", packet);
                    // 将整个数据包打印为u8数组
                    println!("payload: {:?}", payload);
                    // 将有效负载打印为u8数组
                    println!("from_packet: {:?}", from_packet);
                    // 打印侦听器信息:MAC地址，以太类型等，有效载荷是u8的数组
                    println!("---");
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
/// 运行结果
/// ```
/// 开始读取网络数据包:
/// 新数据包:
/// ---
/// packet: [255, 255, 255, 255, 255, 255, 248, 228, 227, 173, 8, 94, 8, 6, 0, 1, 8, 0, 6, 4, 0, 1, 248, 228, 227, 173, 8, 94, 192, 168, 3, 212, 0, 0, 0, 0, 0, 0, 192, 168, 3, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 221, 3, 240, 68]
/// payload: [0, 1, 8, 0, 6, 4, 0, 1, 248, 228, 227, 173, 8, 94, 192, 168, 3, 212, 0, 0, 0, 0, 0, 0, 192, 168, 3, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 221, 3, 240, 68]
/// from_packet: Ethernet { destination: ff:ff:ff:ff:ff:ff, source: f8:e4:e3:ad:08:5e, ethertype: EtherType(2054), payload: [0, 1, 8, 0, 6, 4, 0, 1, 248, 228, 227, 173, 8, 94, 192, 168, 3, 212, 0, 0, 0, 0, 0, 0, 192, 168, 3, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 221, 3, 240, 68] }
/// ---
/// 新数据包:
/// ---
/// packet: [88, 71, 202, 115, 152, 157, 240, 47, 116, 199, 208, 8, 8, 0, 69, 0, 0, 103, 135, 229, 64, 0, 233, 6, 67, 193, 35, 174, 222, 18, 192, 168, 3, 129, 1, 187, 210, 200, 14, 245, 120, 105, 148, 239, 232, 234, 128, 24, 0, 117, 7, 197, 0, 0, 1, 1, 8, 10, 236, 191, 24, 47, 30, 102, 252, 200, 20, 3, 3, 0, 1, 1, 22, 3, 3, 0, 40, 132, 72, 185, 57, 121, 107, 91, 125, 132, 67, 6, 161, 251, 6, 10, 95, 40, 177, 53, 61, 62, 109, 81, 251, 255, 125, 207, 253, 53, 18, 101, 33, 113, 187, 95, 5, 61, 79, 115, 137]
/// payload: [69, 0, 0, 103, 135, 229, 64, 0, 233, 6, 67, 193, 35, 174, 222, 18, 192, 168, 3, 129, 1, 187, 210, 200, 14, 245, 120, 105, 148, 239, 232, 234, 128, 24, 0, 117, 7, 197, 0, 0, 1, 1, 8, 10, 236, 191, 24, 47, 30, 102, 252, 200, 20, 3, 3, 0, 1, 1, 22, 3, 3, 0, 40, 132, 72, 185, 57, 121, 107, 91, 125, 132, 67, 6, 161, 251, 6, 10, 95, 40, 177, 53, 61, 62, 109, 81, 251, 255, 125, 207, 253, 53, 18, 101, 33, 113, 187, 95, 5, 61, 79, 115, 137]
/// from_packet: Ethernet { destination: 58:47:ca:73:98:9d, source: f0:2f:74:c7:d0:08, ethertype: EtherType(2048), payload: [69, 0, 0, 103, 135, 229, 64, 0, 233, 6, 67, 193, 35, 174, 222, 18, 192, 168, 3, 129, 1, 187, 210, 200, 14, 245, 120, 105, 148, 239, 232, 234, 128, 24, 0, 117, 7, 197, 0, 0, 1, 1, 8, 10, 236, 191, 24, 47, 30, 102, 252, 200, 20, 3, 3, 0, 1, 1, 22, 3, 3, 0, 40, 132, 72, 185, 57, 121, 107, 91, 125, 132, 67, 6, 161, 251, 6, 10, 95, 40, 177, 53, 61, 62, 109, 81, 251, 255, 125, 207, 253, 53, 18, 101, 33, 113, 187, 95, 5, 61, 79, 115, 137] }
/// ```

/// 使用超级用户权限运行程序时要小心，确保你理解代码并信任正在使用的库。
///
/// 捕获网络数据包并不复杂，使用Rust和pnet库，可以构建一个安全高效的工具来捕获和分析网络流量。这为网络监控、网络安全和应用程序开发提供了极大的便利。
fn main() {
    simple_2();
}
