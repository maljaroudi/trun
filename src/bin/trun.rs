use std::fs::File;
use std::io::BufReader;
use trun::funcs::*;
fn main() {
    let f = File::open("test.toml").unwrap();
    let mut buffer = BufReader::new(f);
    interpret(&mut buffer);
}
