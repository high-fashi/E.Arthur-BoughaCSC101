use std::fs;
fn main() {
   fs::remove_file("data_txt").unwrap();
   println!("file is removed");
}