fn main() {
    let mut s = String::from("hello");
    s.push_str(", AMERICA");
    println!("{s}");

    let s2 = s;
    println!("{s2}");

    let len = calculate_length(&s2);
    println!("String length is {len}");

    let mut s2 = s2;
    let len = change_and_calc(&mut s2);
    println!("String length is now {len}");
    println!("{s2}");

    let mut s = dangle();
    let len = change_and_calc(&mut s);
    println!("String length is now {len}");

    let binding = String::from("What is the first word");
    let w = first_word(&binding);
    println!("First word {w}");

    let binding = String::from("Whatisthefirstword");
    let w = first_word(&binding);
    println!("First word {w}");


}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn change_and_calc(s: &mut String) -> usize {
    s.push_str(", F*** YEAH, time to save the MOTHERF****** WORLD");
    s.len()
}

fn dangle() -> String {
    let s = String::from("hello");
    s
    // To dangle make s a &mut
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s
}
