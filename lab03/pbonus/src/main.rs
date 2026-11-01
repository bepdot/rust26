use thiserror::Error;

// ----------------------------- P 3

#[derive(Error, Debug)]
enum MathErr {
    #[error("{0} + {1} overflows")]
    AdditionOverflow(u32, u32),
    #[error("{0} * {1} overflows")]
    MultiplicationOverflow(u32, u32),
}

fn checked_addition(x: u32, y: u32) -> Result<u32, MathErr> {
    if x.checked_add(y).is_some() {
        return Ok(x + y);
    }
    Err(MathErr::AdditionOverflow(x, y))
}

fn checked_multiply(x: u32, y: u32) -> Result<u32, MathErr> {
    if x.checked_mul(y).is_some() {
        return Ok(x * y);
    }
    Err(MathErr::MultiplicationOverflow(x, y))
}

fn check_both(x: u32, y: u32) -> Result<u32, MathErr> {
    if !checked_addition(x, y).is_ok() {
        Err(MathErr::AdditionOverflow(x, y))
    } else if !checked_multiply(x, y).is_ok() {
        Err(MathErr::MultiplicationOverflow(x, y))
    } else {
        Ok(1)
    }
}

// ----------------------------- P 4

#[derive(Error, Debug)]
enum CharErr {
    #[error("{0} is not an ASCII character")]
    ErrNotASCII(char),
    #[error("{0} is not a digit")]
    ErrNotDigit(char),
    #[error("{0} is not a base 16 digit")]
    ErrNotB16Digit(char),
    #[error("{0} is not a letter")]
    ErrNotLetter(char),
    #[error("The given character is not a printable character")]
    ErrNotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharErr> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(CharErr::ErrNotLetter(c))
    }
}

fn to_lowercase(c: char) -> Result<char, CharErr> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(CharErr::ErrNotLetter(c))
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
        Err(CharErr::ErrNotASCII(c))
    } else {
        c.to_digit(10).ok_or(CharErr::ErrNotDigit(c))
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharErr> {
    if !c.is_ascii() {
        Err(CharErr::ErrNotASCII(c))
    } else {
        c.to_digit(16).ok_or(CharErr::ErrNotB16Digit(c))
    }
}

fn print_error(err: CharErr) {
    println!("ERR: {}", err);
}

// ----------------------------- P 5

fn first_nondigit(s: &str) -> Option<char> {
    s.chars().find(|&k| k.is_ascii() && !k.is_ascii_digit())
}

fn sum_digits(s: &str) -> Result<u32, CharErr> {
    let mut sum: u32 = 0;
    for c in s.chars() {
        if let Ok(x) = char_to_number(c) {
            sum += x;
        }
    }
    Ok(sum)
}

fn main() {
    println!("{:?}", checked_addition(10, 6));
    println!("{:?}", checked_multiply(676767667, 546464646));
    println!("{:?}", check_both(8, 17));

    println!("-----------------");

    println!("{:?}", to_uppercase('c'));
    println!("{:?}", to_lowercase('T'));
    println!("{:?}", print_char('a'));
    println!("{:?}", char_to_number('3'));
    println!("{:?}", char_to_number('o'));
    println!("{:?}", char_to_number_hex('C'));

    if let Err(e) = char_to_number('[') {
        print_error(e);
    }
    if let Err(e) = to_uppercase('9') {
        print_error(e);
    }

    println!("-----------------");

    println!("{:?}", sum_digits("1234"));
    println!("{:?}", sum_digits("12c4"));
    println!("{:?}", first_nondigit("1111h111k1"));
}
