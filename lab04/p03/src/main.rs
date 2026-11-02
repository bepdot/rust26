use std::io;
use std::fs;

fn read_replace_in_file(fpath: &str) -> Result<(), io:Error> {
    let s = fs::read_to_string(fpath)?;
    let newstr = String::new();

    for w in s.split(' ') {
        let mut newword = w;
        match w {
            "dl" => newword = "domnul",
            _ => continue,
        }
        newstr.push_str(newword);
        newstr.push(' ');
    }

    Ok(())
}

fn main() {
    return
}
