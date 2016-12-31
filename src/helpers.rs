pub const MAC_LEN: usize = 6;

pub fn mac_to_string(mac: &[u8; MAC_LEN]) -> String {
    let strs: Vec<String> = mac.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join(":")
}

pub fn string_to_mac(s: &String) -> [u8; MAC_LEN] {
    assert!(s.len() == 17);
    let mut bytes: [u8; MAC_LEN] = [0; MAC_LEN];
    for i in 0..MAC_LEN {
        let index = i*3;
        match u8::from_str_radix(&s[index..index+2], 16) {
            Ok(a) => bytes[i] = a,
            Err(e) => println!("{} {}", e, &s[index..index+1])
        }
    }
    bytes
}

pub fn ipv4_to_string(ip: &[u8; 4]) -> String {
    let strs: Vec<String> = ip.iter()
        .map(|b| format!("{}", b))
        .collect();
    strs.join(".")
}
/*
 * These functions find if a bit n bits from the LEFT is set
 * from 0 to (n-1)
 */
pub fn bit_set_u64(val: u64, idx: u8) -> bool{
    assert!(idx < 64);
    (val & (1u64 << (63-idx))) == 1
}

pub fn bit_set_u32(val: u32, idx: u8) -> bool{
    assert!(idx < 32);
    (val & (1u32 << (31-idx))) == 1
}

pub fn bit_set_u16(val: u16, idx: u8) -> bool{
    assert!(idx < 16);
    (val & (1u16 << (15-idx))) == 1
}

pub fn bit_set_u8(val: u8, idx: u8) -> bool{
    assert!(idx < 8);
    (val & (1u8 << (7-idx))) == 1
}
