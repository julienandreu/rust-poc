#[derive(Debug)]

enum CustomError<'a> {
    DatabaseError(i32, &'a String),
}

fn extract_ref(val: &String, lifetime: &String) -> Box<String> {
    Box::new(format!("{}-{}", val, lifetime).to_string())
}

fn test2<'a>(rnd: &'a String, error_name: &'a String) -> Result<Box<String>, CustomError<'a>> {
    if rnd.len() < 10 {
        Ok(extract_ref(rnd, error_name))
    } else {
        Err(CustomError::DatabaseError(1, error_name))
    }
}

fn main() {
    let val = "Abc".to_string();
    let a = "Test".to_string();

    test2(&val, &a).unwrap();

    match test2(&val, &a) {
        Ok(out) => {
            println!("Out {}", out);
        }
        Err(e) => {
            println!("Error occurred {:?}", e);
        }
    };

    println!("Out {}", val);
}
