fn shift_string_rot13(s: &str) -> Result<(), String> {
    let mut newstr = String::new();
    for k in s.chars() {
        if !k.is_ascii() {
            return Err(String::from("ERR: Non-ASCII character found."));
        } else {
            let mut newch = k;
            if k.is_ascii_lowercase() {
                newch = (((k as u8 - b'a') + 13) % 26 + b'a') as char;
            } else if k.is_ascii_uppercase() {
                newch = (((k as u8 - b'A') + 13) % 26 + b'A') as char;
            }
            newstr.push(newch);
        }
    }

    println!("{}", newstr);
    Ok(())
}

fn main() {
    if let Err(e) = shift_string_rot13("scientist") {
        println!("{}", e);
    }
    if let Err(e) = shift_string_rot13("fpvrapr uv") {
        println!("{}", e);
    }
    if let Err(e) = shift_string_rot13("🎁🎶🎉👀🎈🎃🍕☕🍉") {
        println!("{}", e);
    }
}
