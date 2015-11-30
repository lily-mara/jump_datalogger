extern crate byteorder;

use byteorder::{ ReadBytesExt, LittleEndian };

use std::io;
use std::io::prelude::*;
use std::fs::File;

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
    let size = match t {
        Type::I32 => BYTES_IN_I32,
        Type::I64 => BYTES_IN_I64,
        Type::F32 => BYTES_IN_F32,
        Type::F64 => BYTES_IN_F64,
    } as u64;

    let num_records = num_bytes / size;

    Ok(match t {
        Type::I32 => TypeVec::I32(try!(load_i32(&mut f, num_records))),
        Type::I64 => TypeVec::I64(try!(load_i64(&mut f, num_records))),
        Type::F32 => TypeVec::F32(try!(load_f32(&mut f, num_records))),
        Type::F64 => TypeVec::F64(try!(load_f64(&mut f, num_records))),
    })
}

fn load_i32<T: ReadBytesExt>(f: &mut T, num_records: u64) -> Result<Vec<i32>, io::Error> {
    let mut out = Vec::new();
    for _ in 0..num_records {
        out.push(try!(f.read_i32::<LittleEndian>()));
    }

    Ok(out)
}

fn load_i64<T: ReadBytesExt>(f: &mut T, num_records: u64) -> Result<Vec<i64>, io::Error> {
    let mut out = Vec::new();
    for _ in 0..num_records {
        out.push(try!(f.read_i64::<LittleEndian>()));
    }

    Ok(out)
}

fn load_f32<T: ReadBytesExt>(f: &mut T, num_records: u64) -> Result<Vec<f32>, io::Error> {
    let mut out = Vec::new();
    for _ in 0..num_records {
        out.push(try!(f.read_f32::<LittleEndian>()));
    }

    Ok(out)
}

fn load_f64<T: ReadBytesExt>(f: &mut T, num_records: u64) -> Result<Vec<f64>, io::Error> {
    let mut out = Vec::new();
    for _ in 0..num_records {
        out.push(try!(f.read_f64::<LittleEndian>()));
    }

    Ok(out)
}
