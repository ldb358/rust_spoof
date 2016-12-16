#[derive(Debug)]
pub struct TCP {
    packet_offset: usize,
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
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
    pub fn new(packet: &Packet, offset: usize) {
        let mut read = Cursor::new(packet.data);
        try!(read.seek(SeekFrom::Start(offset as u64)));
        let mut tcp = TCP {
            packet_offset: offset,
            src_port: 0,
            dst_port: 0,
            seq: 0,
            ack: 0,
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
        tcp.seq = read.read_u32::<NetworkEndian>().unwrap();
        tcp.ack = read.read_u32::<NetworkEndian>().unwrap();
        
        Ok(tcp)
    }

}
