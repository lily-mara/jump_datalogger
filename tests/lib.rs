extern crate jump_datalogger;

use std::f32;
use jump_datalogger::{ load_data_file, Type, TypeVec };

#[test]
fn test_f32_limits_file() {
    let limits = load_data_file("tests/limits.dat", Type::F32).ok().unwrap();
    assert_eq!(limits, TypeVec::F32(vec![f32::MAX, f32::MIN, 3.1415926]));
}
