use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of inner x is: {x}");
    }
    println!("The value of outer x is: {x}");
    println!("How many seconds in 3 hours {THREE_HOURS_IN_SECONDS}");
    tup_example();
    array_example();
}


fn tup_example() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
}


fn array_example() {
    let a = [100; 10];

    println!("Please input your array idx.");

    let mut idx = String::new();

    io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read line");

    let idx: usize = idx.trim().parse().expect("Must enter number");
    let element = a[idx];

    println!("Value of element at idx {idx} is: {element}");
}
