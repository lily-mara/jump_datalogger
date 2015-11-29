extern crate jump_datalogger;

use jump_datalogger::{ load_data_file, Type };

fn main() {
    let x = load_data_file("foo.dat", Type::I32).ok().unwrap();

    println!("{:?}", x)
}
