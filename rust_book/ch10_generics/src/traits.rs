use std::fmt::Display;

// Like an interface - we define what a trait looks like
pub trait Summary {
    // If we don't want a default behavior we add a semicolon after "String"
    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Even to use default behavior you must still implement
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content) 
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


/* Can require traits. This could also be written as notify<T: Summary>(item: &T)
 * The alternative definition is actually better for fns with more vars
 * Eg
 * notify(item1: &impl Summary, item2: &impl Summary)
 * notify<T: Summary>(item1: &T, item2: &T)
 *
 * Can also have multiple trait bounds with +
 * &(impl Summary + Display)
 * <T: Summary + Display>
 *
 * lastly could rewrite with where if there are a lot of traits
 * fn some_fn<T, U>(t: &T, u: &U) -> i32
 * where
 *     T: Display + Clone
 *     U: Clone + Debug
 */
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Can also use impl in returned val
fn returns_summarizable() -> impl Summary {
    Tweet { 
        username: String::from("horse_ebooks"),
        content: String::from("of course this is a tweet"),
        reply: false,
        retweet: false
    }
    // but cannot implement another type in a branch even if it impl Summary
}

// We can conditionally implement methods based on the generic type using this method
struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This also works for traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
