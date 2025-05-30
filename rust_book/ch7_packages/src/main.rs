use crate::garden::vegetables::Asparagus;

pub mod garden;

use rand::Rng;
use std::collections::HashMap;
/* use std::cmp::Ordering;
 * use std::io
 * use std::io::Write */
use std::{cmp::Ordering, io::{self, Write}};

// Can also glob here with use std::collections::*;


fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
}
