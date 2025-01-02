
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").unwrap();
    file.write_all("\nHello class".as_bytes()).unwrap();
    file.write_all("\nThis is the appendage to the document".as_bytes()).unwrap();
    println!("file append success");
}
