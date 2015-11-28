use super::{ u8_to_i32, u8_to_f64 };
use std::{ i32, f64 };

const MAX_I32: [u8; 4] = [0xFF, 0xFF, 0xFF, 0x7F];
const MIN_I32: [u8; 4] = [0x00, 0x00, 0x00, 0x80];
const MAX_F64: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xEF, 0x7F];
const MIN_F64: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xEF, 0xFF];

#[test]
fn test_1i32() {
    assert_eq!(u8_to_i32([0x01, 0x00, 0x00, 0x00]), 1);
}

#[test]
fn test_2i32() {
    assert_eq!(u8_to_i32([0x02, 0x00, 0x00, 0x00]), 2);
}

#[test]
fn test_large_positive() {
    assert_eq!(u8_to_i32([0x0B, 0x2D, 0x7F, 0x50]), 1350511883);
}

#[test]
fn test_large_negative() {
    assert_eq!(u8_to_i32([0xF5, 0xD2, 0x80, 0xAF]), -1350511883);
}

#[test]
fn test_i32_max() {
    assert_eq!(u8_to_i32(MAX_I32), i32::MAX);
}

#[test]
fn test_i32_min() {
    assert_eq!(u8_to_i32(MIN_I32), i32::MIN);
}

#[test]
fn test_pi_f64() {
    assert_eq!(u8_to_f64([0x4A, 0xD8, 0x12, 0x4D, 0xFB, 0x21, 0x09, 0x40]), 3.1415926);
}

#[test]
fn test_f64_max() {
    assert_eq!(u8_to_f64(MAX_F64), f64::MAX);
}

#[test]
fn test_f64_min() {
    assert_eq!(u8_to_f64(MIN_F64), f64::MIN);
}
