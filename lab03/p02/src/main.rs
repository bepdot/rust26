fn checked_addition(x: u32, y: u32) -> u32 {
    match x.checked_add(y) {
        Some(r) => r,
        None => panic!("{} + {} overflows u32", x, y),
    }
}

fn checked_multiply(x: u32, y: u32) -> u32 {
    match x.checked_mul(y) {
        Some(r) => r,
        None => panic!("{} * {} overflows u32", x, y),
    }
}

fn main() {
    println!("{:?}", checked_addition(10, 6));
    println!("{:?}", checked_multiply(676767667, 546464646));
}
