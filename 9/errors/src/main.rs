use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    // panic!("crash and burn");

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        },
    };

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    // let mut f = File::open("hello.txt")?;

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // f.read_to_string(&mut s)?;
    Ok(s)
}
