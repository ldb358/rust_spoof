extern crate pcap;
extern crate spoof;

use spoof::layers::datalink::Ethernet;
use spoof::layers::network::{IPv4};
use spoof::layers::transport::{TCP, UDP};
use spoof::traits::{Chainable, Datalink, Network, NetworkLayer, TransportLayer};
use pcap::{Device, Capture};
use std::io::{self, BufRead, stdout, Write};


fn main() {
    let mut dev: Device = Device{name: String::from(""), desc: None};
    for device in pcap::Device::list().unwrap() {
        println!("Found device! {:?}", device);
        print!("Use?[y/n]:");
        stdout().flush();
        let mut line = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut line).expect("Could not read line");
        if line.starts_with("y") {
            dev.name = device.name;
            dev.desc = device.desc;
            break;
        }
    }

    let mut cap = Capture::from_device(dev).unwrap()
        .promisc(true)
        .snaplen(65535)
        .open().unwrap();
    while let Ok(packet) = cap.next() {
        let e =  Ethernet::new(&packet).unwrap();
        match e.get_next_level() {
            NetworkLayer::IPv4 => {
                let ip = IPv4::new(&packet, e.get_end()).unwrap();
                match ip.get_next_level() {
                    TransportLayer::TCP => {
                        let tcp = TCP::new(&packet, ip.get_end()).unwrap();
                        println!("TCP {}:{} -> {}:{}", 
                                 &ip.get_source_ip(), &tcp.src_port,
                                 &ip.get_destination_ip(), &tcp.dst_port);
                    },
                    TransportLayer::UDP => {
                        let udp = UDP::new(&packet, ip.get_end()).unwrap();
                        println!("UDP {}:{} -> {}:{}", 
                                 &ip.get_source_ip(), &udp.src_port,
                                 &ip.get_destination_ip(), &udp.dst_port);
                    },
                    _ => {}
                }
            },
            _ => {
                println!("{} -> {} Cannot process type", e.get_source_mac(), e.get_destination_mac());
            }
        }
    }
}
