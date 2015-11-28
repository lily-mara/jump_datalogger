extern crate jump_datalogger;

use jump_datalogger::load_data_file;

fn main() {
    let x = load_data_file("bar.dat").ok().unwrap();

    println!("{:?}", x)
}
