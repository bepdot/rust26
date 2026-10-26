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

fn main() {
    println!("{:?}", checked_addition(10, 6));
    println!("{:?}", checked_multiply(676767667, 546464646));
}
