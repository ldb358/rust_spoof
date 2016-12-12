

#[derive(Debug)]
pub struct TCP {
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
