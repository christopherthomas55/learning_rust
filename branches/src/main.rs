fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("Not divisible by 2, 3, 4");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number equals {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 3
        }
    };

    println!("Counter result {result}");
}
