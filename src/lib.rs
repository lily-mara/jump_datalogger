use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::mem;

#[cfg(test)]
mod test;

const BYTES_IN_I32: usize = 4;
const BYTES_IN_I64: usize = 8;
const BYTES_IN_F32: usize = 4;
const BYTES_IN_F64: usize = 8;

#[derive(Debug)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, PartialEq)]
pub enum TypeVec {
    I32(Vec<i32>),
    I64(Vec<i64>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

pub fn load_data_file(filename: &str, t: Type) -> Result<TypeVec, io::Error> {
    let mut f = try!(File::open(filename));
    let num_bytes = try!(f.metadata()).len();
    let num_records = num_bytes / BYTES_IN_F32 as u64;

    Ok(match t {
        Type::I32 => TypeVec::I32(try!(load_i32(&mut f, num_records))),
        Type::I64 => TypeVec::I64(try!(load_i64(&mut f, num_records))),
        Type::F32 => TypeVec::F32(try!(load_f32(&mut f, num_records))),
        Type::F64 => TypeVec::F64(try!(load_f64(&mut f, num_records))),
    })
}

fn load_i32(f: &mut File, num_records: u64) -> Result<Vec<i32>, io::Error> {
    let mut out = Vec::new();
    let mut buf = [0u8; BYTES_IN_I32];

    for _ in 0..num_records {
        try!(f.read(&mut buf));
        out.push(u8_to_i32(buf));
    }

    Ok(out)
}

fn load_i64(f: &mut File, num_records: u64) -> Result<Vec<i64>, io::Error> {
    let mut out = Vec::new();
    let mut buf = [0u8; BYTES_IN_I64];

    for _ in 0..num_records {
        try!(f.read(&mut buf));
        out.push(u8_to_i64(buf));
    }

    Ok(out)
}

fn load_f32(f: &mut File, num_records: u64) -> Result<Vec<f32>, io::Error> {
    let mut out = Vec::new();
    let mut buf = [0u8; BYTES_IN_F32];

    for _ in 0..num_records {
        try!(f.read(&mut buf));
        out.push(u8_to_f32(buf));
    }

    Ok(out)
}

fn load_f64(f: &mut File, num_records: u64) -> Result<Vec<f64>, io::Error> {
    let mut out = Vec::new();
    let mut buf = [0u8; BYTES_IN_F64];

    for _ in 0..num_records {
        try!(f.read(&mut buf));
        out.push(u8_to_f64(buf));
    }

    Ok(out)
}

/// Convert a slice of 4 u8 numbers to a rust i32 number.
fn u8_to_i32(input: [u8; BYTES_IN_I32]) -> i32 {
    unsafe {
        mem::transmute(input)
    }
}

fn u8_to_i64(input: [u8; BYTES_IN_I64]) -> i64 {
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
