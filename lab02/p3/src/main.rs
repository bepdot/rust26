fn add_spaces(s: &mut String, n: i32) {
    for _i in 0..n {
        *s += " ";
    }
}

fn add_str(s: &mut String, s2: &str) {
    *s += s2;
}



fn string_push_number(s: &mut String, n: i32) {
    match n {
        0 => s.push('0'),
        1 => s.push('1'),
        2 => s.push('2'),
        3 => s.push('3'),
        4 => s.push('4'),
        5 => s.push('5'),
        6 => s.push('6'),
        7 => s.push('7'),
        8 => s.push('8'),
        9 => s.push('9'),
        _ => s.push(' ')
    }
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
        string_push_number(s, rev_number % 10);
        
        digit_counter -= 1;
        rev_number /= 10;
        
        if digit_counter % 3 == 0 && digit_counter > 0 {
            *s += "_";
        }
    }
}


fn add_float(s: &mut String, n: f32) {
    let mut m: f32 = n;
    let mut m_int: i32 = m.floor() as i32;
    m -= m_int as f32;

    let mut rev_number_firsthalf: i32 = 1;
    let mut digit_counter: i32 = 0;
    while m_int > 0 {
        rev_number_firsthalf = rev_number_firsthalf * 10 + m_int % 10;
        digit_counter += 1;
        m_int /= 10;
    }
    while rev_number_firsthalf > 10 {
        string_push_number(s, rev_number_firsthalf % 10);
        
        digit_counter -= 1;
        rev_number_firsthalf /= 10;

        if digit_counter % 3 == 0 && digit_counter > 0 {
            *s += "_";
        }
        
    }
    
    *s += ".";

    let mut floating_point_cnt: i32 = 0;

    while m > 0.0 {
        m *= 10.0;
        string_push_number(s, m as i32);
        m -= m.floor();

        floating_point_cnt += 1;
        if floating_point_cnt == 3 {
            break;
        }
    }
}


fn main() {
    let mut s = String::from("");
    add_spaces(&mut s, 40);
    add_str(&mut s, &String::from("I"));
    add_spaces(&mut s, 1);
    add_str(&mut s, &String::from("💚\n"));
    add_spaces(&mut s, 40);
    add_str(&mut s, &String::from("RUST"));

    add_str(&mut s, &String::from("\n\n"));
    add_spaces(&mut s, 5);

    add_str(&mut s, &String::from("Most"));
    add_spaces(&mut s, 12);
    add_str(&mut s, &String::from("crate"));
    add_spaces(&mut s, 5);
    add_integer(&mut s, 306_437_968);
    add_spaces(&mut s, 11);
    add_str(&mut s, &String::from("and"));
    add_spaces(&mut s, 5);
    add_str(&mut s, &String::from("lastest"));
    add_spaces(&mut s, 9);
    add_str(&mut s, &String::from("is"));

    add_str(&mut s, &String::from("\n"));

    add_spaces(&mut s, 10);
    add_str(&mut s, &String::from("downloaded"));
    add_spaces(&mut s, 7);
    add_str(&mut s, &String::from("has"));
    add_spaces(&mut s, 13);
    add_str(&mut s, &String::from("downloads"));
    add_spaces(&mut s, 5);
    add_str(&mut s, &String::from("the"));
    add_spaces(&mut s, 9);
    add_str(&mut s, &String::from("version"));
    add_spaces(&mut s, 4);
    add_float(&mut s, 2.038);

    print!("{}", s);
}