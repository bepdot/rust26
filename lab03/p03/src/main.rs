#[derive(Debug)]
enum MathErr {
    ErrOverflow,
}

fn checked_addition(x: u32, y: u32) -> Result<u32, MathErr> {
    if x.checked_add(y).is_some() {
        return Ok(x + y);
    }
    Err(MathErr::ErrOverflow)
}

fn checked_multiply(x: u32, y: u32) -> Result<u32, MathErr> {
    if x.checked_mul(y).is_some() {
        return Ok(x * y);
    }
    Err(MathErr::ErrOverflow)
}

fn check_both(x: u32, y: u32) -> Result<u32, MathErr> {
    if checked_addition(x, y).is_ok() && checked_multiply(x, y).is_ok() {
        return Ok(1);
    }
    Err(MathErr::ErrOverflow)
}

fn main() {
    println!("{:?}", checked_addition(10, 6));
    println!("{:?}", checked_multiply(676767667, 546464646));
    println!("{:?}", check_both(8, 17));
}
