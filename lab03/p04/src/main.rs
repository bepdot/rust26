#[derive(Debug)]

enum CharErr {
    ErrNotASCII,
    ErrNotDigit,
    ErrNotB16Digit,
    ErrNotLetter,
    ErrNotPrintable
}

fn to_uppercase(c: char) -> Result<char, CharErr> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_uppercase())
    }
    Err(CharErr::ErrNotLetter)
}

fn to_lowercase(c: char) -> Result<char, CharErr> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_lowercase());
    }
    Err(CharErr::ErrNotLetter)
}

fn print_char()


fn main() {
    println!("Hello, world!");
}
