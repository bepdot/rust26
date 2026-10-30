#[derive(Debug)]
enum CharErr {
    ErrNotASCII,
    ErrNotDigit,
    ErrNotB16Digit,
    ErrNotLetter,
    ErrNotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharErr> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(CharErr::ErrNotLetter)
    }
}

fn to_lowercase(c: char) -> Result<char, CharErr> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(CharErr::ErrNotLetter)
    }
}

fn print_char(c: char) -> Result<(), CharErr> {
    if (' '..='~').contains(&c) {
        println!("{}", c);
        Ok(())
    } else {
        Err(CharErr::ErrNotPrintable)
    }
}

fn char_to_number(c: char) -> Result<u32, CharErr> {
    if !c.is_ascii() {
        Err(CharErr::ErrNotASCII)
    } else {
        c.to_digit(10).ok_or(CharErr::ErrNotDigit)
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharErr> {
    if !c.is_ascii() {
        Err(CharErr::ErrNotASCII)
    } else {
        c.to_digit(16).ok_or(CharErr::ErrNotB16Digit)
    }
}

fn print_error(err: CharErr) {
    match err {
        CharErr::ErrNotASCII => println!("ERR: The input character is not an ASCII."),
        CharErr::ErrNotLetter => println!("ERR: The input character is not an alphabetic letter."),
        CharErr::ErrNotDigit => println!("ERR: The input character is not an alphanumeric digit."),
        CharErr::ErrNotB16Digit => {
            println!("ERR: The input character is not a base 16 alphanumeric digit.")
        }
        CharErr::ErrNotPrintable => println!("ERR: The input character is not printable."),
    }
}

fn main() {
    println!("{:?}", to_uppercase('c'));
    println!("{:?}", to_lowercase('T'));
    println!("{:?}", print_char('a'));
    println!("{:?}", char_to_number('3'));
    println!("{:?}", char_to_number('o'));
    println!("{:?}", char_to_number_hex('C'));

    if let Err(e) = char_to_number('[') {
        print_error(e);
    }
}
