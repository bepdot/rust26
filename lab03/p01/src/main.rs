fn next_prime(x: u16) -> Option<u16> {
    if x == u16::MAX {
        return None;
    }
    let u16max: u16 = u16::MAX;
    (x + 1..u16max).find(|&k| check_if_prime(k))
}

fn check_if_prime(k: u16) -> bool {
    for i in 2..k {
        if k.is_multiple_of(i) {
            return false;
        }
    }
    true
}

fn main() {
    println!("{:?}", next_prime(5));
    println!("{:?}", next_prime(65535));
    let mut p = 0;
    while let Some(next) = next_prime(p) {
        println!("{:?}", next);
        p = next;
    }
}
