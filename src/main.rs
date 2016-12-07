extern crate pcap;
extern crate spoof;

use spoof::layers::datalink::Ethernet;
use spoof::layers::network::{IPv4};
use spoof::traits::Chainable;
use pcap::{Device, Capture};



fn main() {
    let main_device = Device{name: String::from("en4"), desc: Some(String::from("Wired connection"))};
    println!("Got {:?}", &main_device);

    let mut cap = Capture::from_device(main_device).unwrap()
        .promisc(true)
        .snaplen(65535)
        .open().unwrap();
    while let Ok(packet) = cap.next() {
        let e =  Ethernet::new(&packet).unwrap();
        if e.p_type == 0x0800u16 {
            println!("{} -> {} And its ipv4", e.get_source_mac(), e.get_destination_mac());
            let ip = IPv4::new(&packet, e.get_end()).unwrap();
            println!("{} -> {}", &ip.get_source_ip(), &ip.get_destination_ip());
        }else{
            println!("{} -> {} Cannot process type", e.get_source_mac(), e.get_destination_mac());
        }
    }
}
