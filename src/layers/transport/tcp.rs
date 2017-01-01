use pcap::{Packet};
use std::io::{Error as IOError, Cursor, Read, Seek, SeekFrom};
use byteorder::{NetworkEndian, ReadBytesExt};
// from this project
use helpers::{bit_set_u16};
use traits::{Chainable};


#[derive(Debug)]
pub struct TCP {
    packet_offset: usize,
    pub src_port: u16,
    pub dst_port: u16,
    pub seq_number: u32,
    pub ack_number: u32,
    pub offset: u16, // first 0-3
    pub reserved: u16, // second 4-6
    pub flags: u16, // 7-15
    // all the flags as easy to access varibles
    pub ns: bool, // 7
    pub cwr: bool, // 8
    pub ece: bool, // 9
    pub urg: bool, // 10
    pub ack: bool, // 11
    pub psh: bool, // 12
    pub rst: bool, // 13
    pub syn: bool, // 14
    pub fin: bool, //15
    pub window_size: u16,
    pub checksum: u16,
    pub urg_pointer: u16,
    pub options: Vec<u32>
}

impl TCP {
    pub fn new(packet: &Packet, offset: usize) -> Result<TCP, IOError> {
        let mut read = Cursor::new(packet.data);
        try!(read.seek(SeekFrom::Start(offset as u64)));
        let mut tcp = TCP {
            packet_offset: offset,
            src_port: 0,
            dst_port: 0,
            seq_number: 0,
            ack_number: 0,
            offset: 0, // first 0-3
            reserved: 0, // second 4-6
            flags: 0, // 7-15
            ns: false, // 7
            cwr: false, // 8
            ece: false, // 9
            urg: false, // 10
            ack: false, // 11
            psh: false, // 12
            rst: false, // 13
            syn: false, // 14
            fin: false, //15
            window_size: 0,
            checksum: 0,
            urg_pointer: 0,
            options: vec!()
        };
        tcp.src_port = read.read_u16::<NetworkEndian>().unwrap();
        tcp.dst_port = read.read_u16::<NetworkEndian>().unwrap();
        tcp.seq_number = read.read_u32::<NetworkEndian>().unwrap();
        tcp.ack_number = read.read_u32::<NetworkEndian>().unwrap();
		let temp = read.read_u16::<NetworkEndian>().unwrap();
        tcp.offset = (0b1111000000000000 & temp) >> 12;
        tcp.reserved = (0b0000111000000000 & temp) >> 9;
        // get the tcp flags
        tcp.flags = 0b0000000111111111 & temp;
        // get whether each flag is set
        tcp.ns = bit_set_u16(temp, 7);
        tcp.cwr = bit_set_u16(temp, 8);
        tcp.ece = bit_set_u16(temp, 9);
        tcp.urg = bit_set_u16(temp, 10);
        tcp.ack = bit_set_u16(temp, 11);
        tcp.psh = bit_set_u16(temp, 12);
        tcp.rst = bit_set_u16(temp, 13);
        tcp.syn = bit_set_u16(temp, 14);
        tcp.fin = bit_set_u16(temp, 15);
        tcp.window_size = read.read_u16::<NetworkEndian>().unwrap();
        tcp.checksum = read.read_u16::<NetworkEndian>().unwrap();
        tcp.urg_pointer = read.read_u16::<NetworkEndian>().unwrap();
        for _ in 0..(&tcp.offset - 5) {
           tcp.options.push(read.read_u32::<NetworkEndian>().unwrap());
        }
        Ok(tcp)
    }
}
impl Chainable for TCP {
    fn get_end(&self) -> usize {
        self.packet_offset + (self.offset * 4) as usize
    }
    
    fn to_binary(&self, higher_levels: &[u8]) -> [u8] {
        let tcp_len = 20 + (4 * self.options.len()); // Tcp min + 4 bytes per option word
        let mut buf: [u8; tcp_len] =  [0; tcp_len];
        NetworkEndian::write_u16(&mut buf, self.src_port);
        NetworkEndian::write_u16(&mut buf, self.dst_port);
        NetworkEndian::write_u32(&mut buf, self.seq_number);
        NetworkEndian::write_u32(&mut buf, self.ack_number);
        buf
    }
}
