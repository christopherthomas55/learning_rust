use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // closure example here
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    // Normally types are inferred in closures but can also type them

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    closure_intro();
    iterator_intro();
}

fn closure_intro () {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("User with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref1 = None;
    let giveaway1 = store.giveaway(user_pref1);
    println!("User with preference {:?} gets {:?}", user_pref1, giveaway1);

    // closures must be first type used as
    let example_closure = |x| x;
    let s = example_closure(String::from("hello world"));
    // This doesn't compile
    // let n = example_closure(5);

    // Closures typicall borrow or borrow mutably
    let mut list = vec![1, 2, 3];
    println!("Before closure! {list:?}");
    let mut borrows_mutably = || list.push(7);
    // Can't println list here because it's in the middle of the closures borrow
    borrows_mutably();
    println!("After calling closure still works! {list:?}");

    // Closures can claim ownership with move keyword
    let list = list;
    println!("Before closure! {list:?}");
    thread::spawn(move || println!("From thread: {list:?}")).join().unwrap();

    // The chapter has lots of good stuff about closure types and what Option uses vs some sorts
    let mut list = [
        Rectangle { width: 10, height:1},
        Rectangle { width: 5, height:10},
        Rectangle { width: 6, height:100932432},
    ];


    // This has the Fn Mut trait so is ok
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    // But this doesn't since it modifies captured value variable out of environment?
    /*let mut sort_operations = vec![];
    let value = String::from("closure called");
    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });*/

    // This would work
    let mut num_operations = 0;
    list.sort_by_key(|r| {
        num_operations += 1;
        r.width
    });

    println!("{list:#?} sorted in {num_operations} operations");
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn iterator_intro() {
    let v1 = vec![1, 2, 3];
    // .into_iter() and .iter_mut() give owned values and mutable references respectively
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }

    // Can call .next() like so
    /*assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);*/

    // Map is another common use
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(vec![2, 3, 4], v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("boot"),
        }
    ];
    for shoe in shoes_in_size(shoes, 10){
        println!("Found shoe! {shoe:?}");
    }
}
