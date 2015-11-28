extern crate jump_datalogger;

use std::f32;
use jump_datalogger::load_data_file;

#[test]
fn test_read_limits_file() {
    let limits = load_data_file("tests/limits.dat").ok().unwrap();
    assert_eq!(limits, vec![f32::MAX, f32::MIN, 3.1415926]);
}
