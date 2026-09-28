use std::cmp;

fn main() {
    println!("coprime pairs of numbers from 1 to 100:");
    for x in 2..101 {
        print!("{} and ", x);
        for y in 2..101 {
            if coprime_check(x, y) == true {
                print!("{}, ", y);
            }
        }
        println!();
        println!("-----------");
    }
}

fn coprime_check(x: i32, y: i32) -> bool {
    let minim: i32 = cmp::min(x, y);

    for i in 2..minim + 1 {
        if x % i == 0 && y % i == 0 {
            return false;
        }
    }
    return true;
}
