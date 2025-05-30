// Lifetimes help us avoid dangling references, like the example below
// This gives a dangling reference since r borrows x and x is dropped when inner scope ends
//
//fn main() {
//    let r;
//    {
//        let x = 5;
//        r = &x;
//    }
//    println!("r: {r}");
//}
//
// We say that the inner scope is the lifetime of x and outer is the lifetime or r


// We have to add lifetimes here because the borrow checker doesn't know the lifetimes
// of the selected str? I really don't know tbh
// It seems like x and y have lifetimes and the compiler will make the lifetime of the output the
// smaller of the tow...the minimuc cover
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn example() {
    let string1 = String::from("abcd");
     {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
// This will not compile due to lifetimes being the shorter of string1 and string2...
/*fn example() {
    let string1 = String::from("abcd");
    let result;
     {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}*/

// Notice that lifetime is a minimum cover based on params, if we specify it for just the result we
// cannot compile. result gets cleaned up because it's life is OvEr
/*fn failed_longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("ehhhhh");
    result.as_str()
}*/

// structs can also hold references if we give them lifetime annotations
// Try it without the annotation
// We do this to have known size at compile time
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn example2() {
    let novel = String::from("Call me Ishmael...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

// NOTE - The rust book has some bits about compiler auto determining lifetime info in fns
// They're called elision rules. The methods below show this off a bit
impl<'a> ImportantExcerpt<'a> {
    // Elision rules can determine lifetime rules here
    fn level(&self) -> i32 {
        3
    }
    // Same here (rule 3 applies since one of the params is self)
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}


