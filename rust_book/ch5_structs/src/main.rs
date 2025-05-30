use std::io;
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
#[derive(Debug)]
struct Vector(i32, i32, i32);
#[derive(Debug)]
struct Rectangle {
    location: Vector,
    width: u32,
    height: u32
}

fn main() {
    let mut user1 = build_user(String::from("me123"), String::from("me123@example.com"));
    user1.email = String::from("anotherone@example.com");
    println!("{}", user1.email);

    let user2 = User {
        active: false,
        ..user1
    };

    println!("User 2 {} is active = {}", user2.email, user2.active);
    // Can't do this since the ..user tuple creation is a borrow action
    //println!("User 1 {} is active = {}", user1.email, user1.active);

    let black = Color(0, 0, 0);
    let origin = Vector(0, 0, 0);


    let mut rect = Rectangle {
        location: dbg!(Vector(0, 0, 0)),
        width: 0,
        height: 0
    };
    println!("Input width then height");
    for i in 0..2 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = input.trim().parse().expect("Not a number!");
        if i==0 {
            rect.width = input;
        } else {
            rect.height = input;
        };
    }
    
    println!("Moving up (1, 1, 1)");
    rect.translate(&Vector(1, 2, 3));
    println!("You have a rectangle like {:?}", rect);
    println!("This area is {}", rect.area());

    let rect2 = Rectangle {width: 20, height: 30, location: Vector(0, 0, 0)};
    println!("Can it hold the test rectangle of {:?}", rect2);
    println!("{}", rect.can_hold(&rect2));

    println!("Also we made this square {:?}", Rectangle::square(10));
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
            location: Vector(0, 0, 0)
        }
    }
    fn translate(&mut self, direction: &Vector) {
        self.location.0 = direction.0;
        self.location.1 = direction.1;
        self.location.2 = direction.2;
    }
    fn can_hold(&mut self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
