use helpers::{string_to_mac, mac_to_string, MAC_LEN};
use traits::{Chainable, Datalink, NetworkLayer};
use pcap::{Packet};
use std::io::{Cursor, Read, Error as IOError};
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};


#[derive(Debug)]
pub struct Ethernet {
    dst_mac: [u8; MAC_LEN],
    src_mac: [u8; MAC_LEN],
    pub p_type: u16
}

impl Ethernet {
    pub fn new(packet: &Packet) -> Result<Ethernet, IOError>  {
        let mut read = Cursor::new(packet.data);
        let mut e = Ethernet {
            dst_mac: [0; MAC_LEN],
            src_mac: [0; MAC_LEN],
            p_type: 0
        };
        try!(read.read_exact(&mut e.dst_mac));
        try!(read.read_exact(&mut e.src_mac));
        e.p_type = read.read_u16::<NetworkEndian>().unwrap();
        Ok(e)
    }

    pub fn get_source_mac(&self) -> String {
        mac_to_string(&self.src_mac)
    }

    pub fn get_destination_mac(&self) -> String {
        mac_to_string(&self.dst_mac)
    }

    pub fn set_source_mac(&mut self, s: String) {
        self.src_mac = string_to_mac(&s);
    }

    pub fn set_destination_mac(&mut self, s: String) {
        self.dst_mac = string_to_mac(&s);
    }
 
}

impl Chainable for Ethernet {
    fn get_end(&self) -> usize {
        // returns the length of ethernet packet
        MAC_LEN * 2 + 2
    }
    
    fn to_binary(&self, vec: &mut Vec<u8>) {
        for val in self.dst_mac.into_iter() {
            vec.write_u8(*val).unwrap();
        }
        for val in self.src_mac.into_iter() {
            vec.write_u8(*val).unwrap();
        }
        vec.write_u16::<NetworkEndian>(self.p_type).unwrap();
    }
}

impl Datalink for Ethernet {
    fn get_next_level(&self) -> NetworkLayer {
        if self.p_type == 0x0800u16 {
            return NetworkLayer::IPv4;
        }
        NetworkLayer::Other
    }
}
