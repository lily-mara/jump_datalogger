use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::mem;

#[cfg(test)]
mod test;

const BYTES_IN_I32: usize = 4;
const BYTES_IN_F32: usize = 4;
const BYTES_IN_F64: usize = 8;

pub fn load_data_file(filename: &str) -> Result<Vec<f32>, io::Error> {
    let mut f = try!(File::open(filename));
    let num_bytes = try!(f.metadata()).len();
    let num_records = num_bytes / BYTES_IN_F32 as u64;
    let mut buf = [0u8; BYTES_IN_F32];
    let mut out = Vec::new();

    for _ in 0..num_records {
        try!(f.read(&mut buf));
        out.push(u8_to_f32(buf));
    }

    Ok(out)
}

/// Convert a slice of 4 u8 numbers to a rust i32 number.
fn u8_to_i32(input: [u8; BYTES_IN_I32]) -> i32 {
    unsafe {
        mem::transmute(input)
    }
}

/// Convert a slice of 8 u8 numbers to a rust f64 number.
fn u8_to_f64(input: [u8; BYTES_IN_F64]) -> f64 {
    unsafe {
        mem::transmute(input)
    }
}

/// Convert a slice of 4 u8 numbers to a rust f32 number.
fn u8_to_f32(input: [u8; BYTES_IN_F32]) -> f32 {
    unsafe {
        mem::transmute(input)
    }
}
