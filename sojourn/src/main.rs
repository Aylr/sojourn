use std::env;
use std::path;
use std::fs;

fn main() {
    let base_dir = "/Users/taylor/repos/journal/";

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let note = &args[1];
    let date = &args[2];

    println!("note: {}", note);
    println!("date: {}", date);
    let base_path = path::Path::new(base_dir);
    let filepath = base_path.join("rust.md");
    println!("Attempting to open {:?}", filepath);

    // let contents = fs::read_to_string(filepath);
    // println!("file contains: {}", contents);

}
