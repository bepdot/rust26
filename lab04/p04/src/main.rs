use std::fs;
use std::io;

fn read_print_hosts(fpath: &str) -> Result<(), io::Error> {
    let s = fs::read_to_string(fpath)?;
    println!("unpacked");
    for k in s.lines() {
        println!("{}", k.trim());
        if k.is_empty() || k.starts_with("#") { continue; }
        let mut col = k.split(' ');
        if let Some(c1) = col.next() && let Some(c2) = col.next() {
            println!("{} => {}", c1, c2);
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = read_print_hosts("C:\\Windows\\System32\\drivers\\etc\\hosts") {
        println!("err: {}", e);
    }
}
