use serde_derive::Deserialize;
use std::fs;
use std::io;

#[derive(Clone, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn main() -> Result<(), io::Error> {
    let fread = fs::read_to_string("studenti.json")?;
    // the json used isn't a valid json format
    // wonder how that could be fixed and worked with

    let mut oldest: Option<Student> = Some(Student {
        name: String::new(),
        phone: String::new(),
        age: 0,
    });
    let mut youngest: Option<Student> = Some(Student {
        name: String::new(),
        phone: String::new(),
        age: 999,
    });

    for k in fread.lines() {
        let s: Student = match serde_json::from_str(k) {
            Ok(s) => s,
            Err(_) => continue,
        };

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
