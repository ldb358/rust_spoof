use pcap::{Packet};
use std::io::{Error as IOError, Cursor, Read, Seek, SeekFrom};
use byteorder::{NetworkEndian, ReadBytesExt};
// from this project
use helpers::{bit_set_u16};
use traits::{Chainable};


#[derive(Debug)]
pub struct UDP {
    packet_offset: usize,
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl UDP {
    pub fn new(packet: &Packet, offset: usize) -> Result<UDP, IOError> {
        let mut read = Cursor::new(packet.data);
        try!(read.seek(SeekFrom::Start(offset as u64)));
        let mut udp = UDP {
            packet_offset: offset,
            src_port: 0,
            dst_port: 0,
            length: 0,
            checksum: 0
        };
        udp.src_port = read.read_u16::<NetworkEndian>().unwrap();
        udp.dst_port = read.read_u16::<NetworkEndian>().unwrap();
        udp.length = read.read_u16::<NetworkEndian>().unwrap();
        udp.checksum = read.read_u16::<NetworkEndian>().unwrap();
        Ok(udp)
    }

}

impl Chainable for UDP {
    fn get_end(&self) -> usize {
        self.packet_offset + self.length as usize
    }
}
