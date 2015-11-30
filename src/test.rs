use std::io::Cursor;
use std::{ i32, i64, f32, f64 };
use super::{ load_i32, load_i64, load_f32, load_f64 };

const MAX_I32: [u8; 4] = [0xFF, 0xFF, 0xFF, 0x7F];
const MIN_I32: [u8; 4] = [0x00, 0x00, 0x00, 0x80];
const MAX_I64: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F];
const MIN_I64: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
const MAX_F32: [u8; 4] = [0xFF, 0xFF, 0x7F, 0x7F];
const MIN_F32: [u8; 4] = [0xFF, 0xFF, 0x7F, 0xFF];
const MAX_F64: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xEF, 0x7F];
const MIN_F64: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xEF, 0xFF];

#[test]
fn test_i32_min() {
    let mut x = Cursor::new(MIN_I32);
    assert_eq!(load_i32(&mut x, 1).ok().unwrap(), vec![i32::MIN]);
}

#[test]
fn test_i32_max() {
    let mut x = Cursor::new(MAX_I32);
    assert_eq!(load_i32(&mut x, 1).ok().unwrap(), vec![i32::MAX]);
}

#[test]
fn test_i64_min() {
    let mut x = Cursor::new(MIN_I64);
    assert_eq!(load_i64(&mut x, 1).ok().unwrap(), vec![i64::MIN]);
}

#[test]
fn test_i64_max() {
    println!("{}", i64::MAX);
    let mut x = Cursor::new(MAX_I64);
    assert_eq!(load_i64(&mut x, 1).ok().unwrap(), vec![i64::MAX]);
}

#[test]
fn test_f32_min() {
    let mut x = Cursor::new(MIN_F32);
    assert_eq!(load_f32(&mut x, 1).ok().unwrap(), vec![f32::MIN]);
}

#[test]
fn test_f32_max() {
    let mut x = Cursor::new(MAX_F32);
    assert_eq!(load_f32(&mut x, 1).ok().unwrap(), vec![f32::MAX]);
}

#[test]
fn test_f64_min() {
    let mut x = Cursor::new(MIN_F64);
    assert_eq!(load_f64(&mut x, 1).ok().unwrap(), vec![f64::MIN]);
}

#[test]
fn test_f64_max() {
    let mut x = Cursor::new(MAX_F64);
    assert_eq!(load_f64(&mut x, 1).ok().unwrap(), vec![f64::MAX]);
}
