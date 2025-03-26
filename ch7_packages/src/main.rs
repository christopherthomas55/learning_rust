use crate::garden::vegetables::Asparagus;

pub mod garden;

use rand::Rng;
use std::collections::HashMap;
use std::{cmp::Ordering, io::{self, Write}};


fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
}
