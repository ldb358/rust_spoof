use pcap::{Packet};
use std::io::{Error as IOError, Cursor, Read, Seek, SeekFrom};
use byteorder::{NetworkEndian, ReadBytesExt};
// from this project
use helpers::{ipv4_to_string};
use traits::{Chainable, Network, TransportLayer};

#[derive(Debug)]
pub struct IPv4 {
    packet_offset: usize,
    pub version: u8, // first 4
    pub header_length: u8, // last 4
    pub p_type: u8,
    pub length: u16,
    pub id: u16,
    pub flags: u8, // first 3
    pub fragment_offset: u16, //last 13
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
    pub options: Vec<u32>
}


impl IPv4 {
    pub fn new(packet: &Packet, offset: usize) -> Result<IPv4, IOError> {
        let mut read = Cursor::new(packet.data);
        try!(read.seek(SeekFrom::Start(offset as u64)));
        let mut ip = IPv4 {
            packet_offset: offset,
            version: 0,
            header_length: 0,
            p_type: 0,
            length: 0,
            id: 0,
            flags: 0,
            fragment_offset: 0,
            ttl: 0,
            protocol: 0,
            checksum: 0,
            src_ip: [0; 4],
            dst_ip: [0; 4],
            options: vec!()
        };
        let temp: u8 = read.read_u8().unwrap();
        ip.version = (0b11110000 & temp) >> 4;
        ip.header_length = 0b00001111 & temp;
        ip.p_type = read.read_u8().unwrap();
        ip.length = read.read_u16::<NetworkEndian>().unwrap();
        ip.id = read.read_u16::<NetworkEndian>().unwrap();
        let flag_frag = read.read_u16::<NetworkEndian>().unwrap();
        ip.flags = ((0b11100000 & flag_frag) >> 5) as u8;
        ip.fragment_offset = 0b00011111 & flag_frag;
        ip.ttl = read.read_u8().unwrap();
        ip.protocol = read.read_u8().unwrap();
        ip.checksum = read.read_u16::<NetworkEndian>().unwrap();
        try!(read.read_exact(&mut ip.src_ip));
        try!(read.read_exact(&mut ip.dst_ip));
        for _ in 0..(&ip.header_length - 5) {
           ip.options.push(read.read_u32::<NetworkEndian>().unwrap());
        }
        Ok(ip)
    }

    pub fn get_source_ip(&self) -> String {
        ipv4_to_string(&self.src_ip)
    }

    pub fn get_destination_ip(&self) -> String {
        ipv4_to_string(&self.dst_ip)
    }
}


impl Chainable for IPv4 {
    fn get_end(&self) -> usize {
        self.packet_offset + (self.header_length * 4) as usize
    }
}

impl Network for IPv4 {
    fn get_next_level(&self) -> TransportLayer {
        match self.protocol {
            6 => TransportLayer::TCP,
            17 => TransportLayer::UDP,
            _ => TransportLayer::Other
        }
        
    }
}
