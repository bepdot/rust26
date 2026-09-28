fn main() {
    println!("primes from 1 to 100:");
    for n in 1..101 {
        if primechecker(n) == true {
            print!("{} ", n as i32);
        }
    }
}

fn primechecker(value: i32) -> bool {
    for n in 2..value-1 {
        if value % n == 0 {
            return false;
        }
    }
    return true;
}
