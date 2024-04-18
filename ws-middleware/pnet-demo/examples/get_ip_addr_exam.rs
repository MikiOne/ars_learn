use pnet::datalink;

fn main() {
    let interface = datalink::interfaces().into_iter()
        .find(|interface| interface.name == "en0")
        .unwrap();
    println!("en0 interface: {}", interface);

    for iface in datalink::interfaces() {
        for ip_network in iface.ips {
            if ip_network.is_ipv4() {
                println!("{}", ip_network.ip());
            }
        }
    }
}