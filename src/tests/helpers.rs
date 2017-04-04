use super::super::helpers::{mac_to_string, string_to_mac, ipv4_to_string, bit_set_u64, bit_set_u32, bit_set_u16, bit_set_u8};

#[test]
fn test_mac_to_string_single() {
    let mac = [2; 6];
    let string = mac_to_string(&mac);
    assert_eq!(string, "02:02:02:02:02:02");
}

#[test]
fn test_mac_to_string_double() {
    let mac = [0x1f; 6];
    let string = mac_to_string(&mac);
    assert_eq!(string, "1F:1F:1F:1F:1F:1F");
}


#[test]
fn test_string_to_mac_single() {
    let mac = "02:02:02:02:02:02";
    let arr = string_to_mac(&String::from(mac));
    assert_eq!(arr, [2; 6]);
}

#[test]
fn test_string_to_mac_double() {
    let mac = "1f:1f:1f:1f:1f:1f";
    let arr = string_to_mac(&String::from(mac));
    assert_eq!(arr, [0x1f; 6]);
}

#[test]
fn test_string_to_mac_upper() {
    let mac = "1F:1F:1F:1F:1F:1F";
    let arr = string_to_mac(&String::from(mac));
    assert_eq!(arr, [0x1f; 6]);
}

#[test]
fn test_ipv4_to_string() {
    let mac = [10; 4];
    let string = ipv4_to_string(&mac);
    assert_eq!(string, "10.10.10.10");
}

#[test]
fn test_bit_set_u64_first_pos() {
    let val: u64 = 0b1 << 63;    
    assert!(bit_set_u64(val, 0));
}

#[test]
fn test_bit_set_u64_last() {
    let val: u64 = 0b1;
    assert!(bit_set_u64(val, 63));
}

#[test]
fn test_bit_set_u64_first_neg() {
    let val: u64 = 0b1 << 62;    
    assert!(!bit_set_u64(val, 0));
}

#[test]
fn test_bit_set_u64_last_neg() {
    let val: u64 = 0b10;    
    assert!(!bit_set_u64(val, 63));
}

#[test]
#[should_panic]
fn test_bit_set_u64_overbound() {
    let val: u64 = 0b10;    
    assert!(!bit_set_u64(val, 64));
}

#[test]
fn test_bit_set_u32_first_pos() {
    let val: u32 = 0b1 << 31;    
    assert!(bit_set_u32(val, 0));
}

#[test]
fn test_bit_set_u32_last() {
    let val: u32 = 0b1;
    assert!(bit_set_u32(val, 31));
}

#[test]
fn test_bit_set_u32_first_neg() {
    let val: u32 = 0b1 << 30;    
    assert!(!bit_set_u32(val, 0));
}

#[test]
fn test_bit_set_u32_last_neg() {
    let val: u32 = 0b10;    
    assert!(!bit_set_u32(val, 31));
}

#[test]
#[should_panic]
fn test_bit_set_u32_overbound() {
    let val: u32 = 0b10;    
    assert!(!bit_set_u32(val, 32));
}

#[test]
fn test_bit_set_u16_first_pos() {
    let val: u16 = 0b1 << 15;    
    assert!(bit_set_u16(val, 0));
}

#[test]
fn test_bit_set_u16_last() {
    let val: u16 = 0b1;
    assert!(bit_set_u16(val, 15));
}

#[test]
fn test_bit_set_u16_first_neg() {
    let val: u16 = 0b1 << 14;    
    assert!(!bit_set_u16(val, 0));
}

#[test]
fn test_bit_set_u16_last_neg() {
    let val: u16 = 0b10;    
    assert!(!bit_set_u16(val, 15));
}

#[test]
#[should_panic]
fn test_bit_set_u16_overbound() {
    let val: u16 = 0b10;    
    assert!(!bit_set_u16(val, 32));
}

#[test]
fn test_bit_set_u8_first_pos() {
    let val: u8 = 0b10000000;    
    assert!(bit_set_u8(val, 0));
}

#[test]
fn test_bit_set_u8_last() {
    let val: u8 = 0b1;
    assert!(bit_set_u8(val, 7));
}

#[test]
fn test_bit_set_u8_first_neg() {
    let val: u8 = 0b1 << 6;    
    assert!(!bit_set_u8(val, 0));
}

#[test]
fn test_bit_set_u8_last_neg() {
    let val: u8 = 0b10;    
    assert!(!bit_set_u8(val, 7));
}

#[test]
#[should_panic]
fn test_bit_set_u8_overbound() {
    let val: u8 = 0b10;    
    assert!(!bit_set_u8(val, 8));
}
