pub const MAC_LEN: usize = 6;

/// Converts a mac address from array to a string representation
///
/// ```
/// use spoof::helpers::mac_to_string;
/// let mac = [0x1f; 6];
/// let string = mac_to_string(&mac);
/// assert_eq!(string, "1F:1F:1F:1F:1F:1F");
/// ```
pub fn mac_to_string(mac: &[u8; MAC_LEN]) -> String {
    let strs: Vec<String> = mac.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join(":")
}


/// Converts a mac address from string to array representation
///
/// ```
/// use spoof::helpers::string_to_mac;
/// let mac = "1f:1f:1f:1f:1f:1f";
/// let arr = string_to_mac(&String::from(mac));
/// assert_eq!(arr, [0x1f; 6]);
/// ```
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

/// Converts ipv4 from array to string representation
///
/// ```
/// use spoof::helpers::ipv4_to_string;
/// let mac = [10; 4];
/// let string = ipv4_to_string(&mac);
/// assert_eq!(string, "10.10.10.10");
/// ```

pub fn ipv4_to_string(ip: &[u8; 4]) -> String {
    let strs: Vec<String> = ip.iter()
        .map(|b| format!("{}", b))
        .collect();
    strs.join(".")
}

/// Get if the bit idx from the left is set
///
/// ```
/// use spoof::helpers::bit_set_u64;
/// let val: u64 = 0b1010; 
/// assert!(bit_set_u64(val, 62));
/// assert!(!bit_set_u64(val, 63));
/// ```
pub fn bit_set_u64(val: u64, idx: u8) -> bool{
    assert!(idx < 64);
    (val & (1u64 << (63-idx))) >= 1
}

pub fn bit_set_u32(val: u32, idx: u8) -> bool{
    assert!(idx < 32);
    (val & (1u32 << (31-idx))) >= 1
}

pub fn bit_set_u16(val: u16, idx: u8) -> bool{
    assert!(idx < 16);
    (val & (1u16 << (15-idx))) >= 1
}

pub fn bit_set_u8(val: u8, idx: u8) -> bool{
    assert!(idx < 8);
    (val & (1u8 << (7-idx))) >= 1
}
