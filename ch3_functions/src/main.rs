fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(10, 'g');
    expression_example();
}

fn another_function(x: i32) {
    println!("Another one with x={x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression_example() {
    let y = {
        let x: u32 = 100;
        x*x
    };
    println!("The value of y is {y}");
    
    fn plus(x: i32, y: i32) -> i32 {
        x + y
    }
    let x = plus(109842, 029384);
    println!("The value of this weird fn is is {x}");

}

