use std::fs;
use std::io;

fn read_replace_in_file(fpath: &str) -> Result<(), io::Error> {
    let s = fs::read_to_string(fpath)?;
    let mut newstr = String::new();

    for w in s.split(' ') {
        let mut newword = w;
        match newword {
            "pt" => newword = "pentru",
            "ptr" => newword = "pentru",
            "dl" => newword = "domnul",
            "dna" => newword = "doamna",
            _ => newword = w,
        }
        newstr.push_str(newword);
        newstr.push(' ');
    }

    println!("{}", newstr);

    Ok(())
}

fn main() {
    if let Err(e) = read_replace_in_file("tfile.txt") {
        println!("err: {}", e);
    }
}
