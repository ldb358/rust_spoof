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
