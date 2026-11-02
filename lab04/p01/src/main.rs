use std::io;
use std::fs;

fn read_file_get_longest_bytes(fpath: &str) -> Result<(), io::Error> {
    let s = fs::read_to_string(fpath)?;
    let mut longest_bytes = "";
    for line in s.lines() {
        if line.len() > longest_bytes.len() {
            longest_bytes = line;
        }
    }
    println!("longest by bytes: {:?}", longest_bytes);
    Ok(())
}

fn read_file_get_longest_chars(fpath: &str) -> Result<(), io::Error> {
    let s = fs::read_to_string(fpath)?;
    let mut longest_chars = "";
    for line in s.lines() {
        if line.chars().count() > longest_chars.chars().count() {
            longest_chars = line;
        }
    }
    println!("longest by length: {:?}", longest_chars);
    Ok(())
}

fn main() {
    if let Err(e) = read_file_get_longest_chars("science.txt") {
        println!("err: {}", e);
    }
    if let Err(e) = read_file_get_longest_bytes("science.txt") {
        println!("err: {}", e);
    }
}
