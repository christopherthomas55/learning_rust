use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // ? Operator means function must expect result or enum!
    // We have to change the return type of main (which is restricted like c to return ints) to handle
    // We will learn about Box<dyn Error> later
    let greeting_file = File::open("hello.txt")?;
    Ok(())
    

    //even_simpler_panics();
    //simple_example_results();
    //complicated_example_results
    //example_panic();
}


fn complicated_example_results() {
    /* Get to know the Result enum
     *   enum Result<T, E> {
     *   Ok(T),
     *   Err(E),
     *   }
     */
    println!("Please input file name");

    let mut file_name = String::new();

    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");
    let file_name = file_name.trim();

    let file_example = File::open(&file_name);
    let file_example = match file_example {
        Ok(file) => {println!("{file:?} exists"); file},
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(fc) => {println!("{fc:?} created"); fc},
                Err(e) => panic!("Problem creating file {e:?}"),
            },
            other_error => {
                panic!("Problem opening file {other_error:?}");
            }
        }
    };
    println!("{file_example:?}");
}

fn simple_example_results() {
    // This does the same as the complicated version but simplifies using built in Result<T, E> methods
    println!("Please input file name");

    let mut file_name = String::new();

    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");
    let file_name = file_name.trim();

    // |X| is a closure - we'll learn more in ch 13!
    let file_example = File::open(&file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&file_name).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn even_simpler_panics() {
    // This shows off simple panics
    println!("Please input file name");

    let mut file_name = String::new();

    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");
    let file_name = file_name.trim();

    // Panics on no file
    //let greeting_file = File::open(file_name).unwrap();

    // Panics on no file but with error message. This is usually better
    // I am finiding it difficult to format in an expect stmt since it expects a fixed length &str
    let greeting_file = File::open(file_name).expect("File not found");
}

fn example_panic() {
    let mut v = vec![];
    for i in 1..90 {
        v.push(i);
    }
    v[99];
}

// This is what avoiding the ? operator looks like
fn long_way_read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

// Look how easy this is when we use the ? operator!
fn easy_way_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file= File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    // Cound even chain calls like File::open("hello.txt")?.read_to_string(&mut username)?
    Ok(username)
    // This does what std::fs fs::read_to_string does ha!
}

// ? Operator can also be used to return None in Option cases, but doesn't mix with Result
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


fn dumb_result_test(val: u8) -> Result<char, u8> {
    // Tested - the result enum must have an OK(param) and an Err(type)
    if val > 5 {
        return Ok('b')
    }
    Err(val)
}
