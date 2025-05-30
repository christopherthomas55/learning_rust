fn main() {
    vectors();
    strings();
    hashmap();
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
    None
}

fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    println!("Vector is {v:?}");

    let seventh: Option<&i32> = v.get(6);
    match seventh {
        Some(seventh) => println!("Seventh is {seventh}"),
        None => println!("Nothing in seventh"),
    }

    for i in 5..9 {
        v.push(i);
    }
    println!("Vector is now {v:?}");

    let seventh: &i32 = &v[6];
    println!("Seventh is {seventh}");

    let seventh: Option<&i32> = v.get(6);
    match seventh {
        Some(seventh) => println!("Seventh is {seventh}"),
        None => println!("Nothing in seventh"),
    }

    for i in &mut v {
        *i *= 20;
    }

    for i in &v {
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::None
    ];

}

fn strings() {
    let mut s = String::new();
    let data= "initial";
    let s = data.to_string();
    let s = "initial".to_string();
    let mut s = String::from("initial");
    let s2 = " string2";
    s.push_str(s2);
    s.push('!');
    s.push_str("!");
    println!("s2 is {s2}");
    println!("s1 is {s}");

    let s3 = s + &s2;
    println!("s3 is {s3}");

    let s = "Sike!";
    let s = format!("{s}-{s2}-{s3}");
    println!("s is {s}");

    // there's some more about unicode that is hard to replicate here - I don't have neovim
    // pasting set up. We cannot readily use and index to return a value - do we use bytes or go by chars?
    // Unicode chars can be 1-4 bytes in length
    // But here's how to loop through and index strings.
    let s = &s[0..4];
    println!("s is {s}");

    for c in s.chars(){
        println!("{c}");
    }
    for c in s.bytes(){
        println!("{c}");
    }
}


use std::collections::HashMap;

fn hashmap() {
    let mut scores = HashMap::new();
    let blue_team = String::from("Blue");
    let yellow_team = String::from("Yellow");
    scores.insert(blue_team, 10);
    scores.insert(yellow_team, 50);
    // Overwrites by default
    let yellow_team = String::from("Yellow");
    scores.insert(yellow_team, 25);

    // Hashmaps borrow typs without Copy traits println!("{blue_team}");

    let blue_team = String::from("Blue");
    let score = scores.get(&blue_team).unwrap_or(&0);
    println!("{blue_team} score is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Can check if entry with following code
    let red_team = String::from("Red");
    scores.entry(red_team).or_insert(100);
    let red_team = String::from("Red");
    scores.entry(red_team).or_insert(999);
    println!("{scores:?}");

    // Must dereference to update entry using old value
    let mut words = HashMap::new();
    for word in "hello world and hello to you especially CJ".split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{words:?}");
}
