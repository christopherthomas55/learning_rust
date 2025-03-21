fn testing() {
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

    while counter >= 0 {
        println!("{counter}");
        counter -= 1
    }
    
    for number in 1..10 {
        println!("{number}");
    }
}


fn christmas() {
    let a = ["1 thing", "2 things", "3 things", "4 things", "5 things"];
    for number in 1..12 {
        println!("For the {number} day of Christmas my true love gave to me");
        if number > 4 {
            println!("I can't afford more than {number} days of Christmas");
            break
        }
        for x in (0..number).rev() {
            println!("{}", a[x]);
        }

    }
}


fn main() {
    testing();
    christmas();
}
