use std::env;
use std::fs::File;
use std::io::BufReader;
use trun::funcs::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong Number of Arguments Expected File Name Only");
        return;
    }
    let f = File::open(&args[1]).unwrap();
    let mut buffer = BufReader::new(f);
    interpret(&mut buffer).unwrap();
}
