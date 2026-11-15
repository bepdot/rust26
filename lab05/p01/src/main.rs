use std::fs;
use std::io;

#[derive(Clone)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn parse_line(line: &str) -> Option<Student> {
    let mut parts = line.split(',');
    let name = parts.next()?;
    let phone = parts.next()?;
    let age = parts.next()?.trim().parse().ok()?;
    Some(Student {
        name: name.to_string(),
        phone: phone.to_string(),
        age,
    })
}

fn main() -> Result<(), io::Error> {
    let fread = fs::read_to_string("studenti.txt")?;

    let mut oldest: Option<Student> = Some(
        Student {
            name: String::new(),
            phone: String::new(),
            age: 0,
        }
    );
    let mut youngest: Option<Student> = Some(
        Student {
            name: String::new(),
            phone: String::new(),
            age: 999,
        }
    );

    for k in fread.lines() {
        let Some(s) = parse_line(k) else { continue };

        let replaceold = match &oldest {
            Some(o) => s.age > o.age,
            None => true,
        };
        if replaceold {
            oldest = Some(s.clone());
        }

        let replaceyng = match &youngest {
            Some(y) => s.age < y.age,
            None => true,
        };
        if replaceyng {
            youngest = Some(s.clone());
        }
    }

    if let Some(o) = &oldest {
        println!(
            "Oldest student: name - {} | phone - {} | age - {}",
            o.name, o.phone, o.age
        );
    }
    if let Some(y) = &youngest {
        println!(
            "Youngest student: name - {} | phone - {} | age - {}",
            y.name, y.phone, y.age
        );
    }

    Ok(())
}
