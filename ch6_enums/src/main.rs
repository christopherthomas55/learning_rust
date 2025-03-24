use rand::Rng;

#[derive(Debug)]
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}


enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


#[derive(Debug)]
enum USState  {
    Texas,
    AllOthers
}


#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter(USState),
}


fn main() {
    // Enum intro
    let home = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    route(&home);
    route(&six);

    let m = Message::Write(String::from("hello"));
    m.call();


    let some_num = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: Option<i32> = Some(5);
    let y: Option<i32> = Some(5);
    let six = plus_one(x);
    let none = plus_one(None);



    let example_coin = Coin::Quarter(USState::Texas);
    let res = val_in_cents(&example_coin);
    println!("Coin {example_coin:?} has value {res}");

    let dice_roll = rand::thread_rng().gen_range(1..=12);
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        _ => (),
    };

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }


    // The following are equal
    let mut count = 0;
    match &example_coin {
        Coin::Quarter(state) => println!("State Quarter from {state:?}!"),
        _ => count += 1
    }

    let mut count = 0;
    if let Coin::Quarter(state) = &example_coin {
        println!("State Quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("{:?}", describe_state_quarter(example_coin));
    let example_coin = Coin::Quarter(USState::AllOthers);
    println!("{:?}", describe_state_quarter(example_coin));


}

impl USState {
    fn did_exist(&self, year: u16) -> bool {
        match self {
            USState::Texas => {year >= 1850},
            USState::AllOthers => {year >= 1776},
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    /* Worse way of doing it
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    }; */
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.did_exist(1800) {
        Some(format!("{state:?} is old news bud!"))
    } else {
        Some(format!("{state:?} is the future!"))
    }
}

fn route(ip: &IpAddr) {
    println!("Hello, route {ip:?}");
}


impl Message {
    fn call(&self) {
    }

}


fn add_hat() {}
fn remove_hat() {}
fn reroll() {}



fn val_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {println!("Found quarter for state {state:?}"); 25}
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
