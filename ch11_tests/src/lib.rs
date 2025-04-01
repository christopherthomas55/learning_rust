use std::ops::Add;

pub fn add<T: Add<Output = T>>(left: T, right: T) -> T {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100, got {value}");
        }
         Guess {value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("This fails the test!");
    }

    #[test]
    fn larger_holds_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 6,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // assert! can also print a string on failure
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was {result}");
    }

    // Can see if test should panic and check the str message in panic too
    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Also can use the Result enum in tests
    // But make sure not to use the question mark operator...it will auto convert the Err type if
    // possible
    #[test]
    fn it_works_2() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two must equal 4"))
        }
    }

    // Can also ignore tests unless specifically requested
    #[test]
    #[ignore]
    fn expensive_test() {
    }
}
