#[derive(Debug)]
enum CharErr {
    ErrNotASCII,
    ErrNotDigit,
}

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

fn char_to_number(c: char) -> Result<u32, CharErr> {
    if !c.is_ascii() {
        Err(CharErr::ErrNotASCII)
    } else {
        c.to_digit(10).ok_or(CharErr::ErrNotDigit)
    }
}

fn main() {
    println!("{:?}", sum_digits("1234"));
    println!("{:?}", sum_digits("12c4"));
    println!("{:?}", first_nondigit("1111h111k1"));
}
