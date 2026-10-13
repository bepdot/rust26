fn add_spaces(s: &mut String, n: i32) {
    for _i in 0..n {
        *s += " ";
    }
}

fn add_str(s: &mut String, s2: &str) {
    *s += s2;
}

fn add_integer(s: &mut String, n: i32) {
    let mut m: i32 = n;
    let mut rev_number: i32 = 1;
    let mut digit_counter: i32 = 0;
    while m > 0 {
        rev_number = rev_number * 10 + m % 10;
        digit_counter += 1;
        m /= 10;
    }
    while rev_number > 10 {
        s.push(((rev_number % 10) as u8) as char);
        if digit_counter % 3 == 0 && digit_counter > 0 {
            *s += "_";
        }
        rev_number /= 10;
    }
}

fn add_float(s: &mut String, n: f32) {
    let mut m: f32 = n;
    let mut rev_number: f32 = 1.0;
    let mut digit_counter: i32 = 0;
    while m >= 1.0 {
        rev_number = rev_number * 10.0 + m % 10.0;
        digit_counter += 1;
        m /= 10.0;
    }
    while rev_number > 10.0 {
        s.push(((rev_number % 10.0) as u8) as char);
        if digit_counter % 3 == 0 && digit_counter > 0 {
            *s += "_";
        }
        rev_number /= 10.0;
    }
    
    *s += ".";

    while m > 0.0 {
        m *= 10.0;
        s.push((m as u8) as char);
        m -= m.floor();
    }
}

fn main() {
    let mut s = String::from("");
    add_spaces(&mut s, 30);
    add_str(&mut s, &String::from("I"));
    add_spaces(&mut s, 1);
    add_str(&mut s, &String::from("💚\n"));
    add_spaces(&mut s, 30);
    add_str(&mut s, &String::from("RUST"));

    add_str(&mut s, &String::from("\n\n"));
    add_spaces(&mut s, 5);
    add_str(&mut s, &String::from("Most downloaded crate has "));
    add_integer(&mut s, 306_437_968);
    add_str(&mut s, &String::from(" downloads and the latest version is "));
    add_float(&mut s, 2.038);

    print!("{}", s);
}