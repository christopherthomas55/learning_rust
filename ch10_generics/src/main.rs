mod generics_1;
mod traits;
use traits::{Summary, Tweet, NewsArticle, notify};
mod lifetimes;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = generics_1::largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![34, 50, 25, 100, 65, 9807234];
    let result = generics_1::largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['q', 'g', 't', 'a'];
    let result = generics_1::largest(&char_list);
    println!("The largest char is {result}");

    generics_1::example();

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course this is a tweet"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    // This bit shows off a default implementation of trait summarize
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley cup!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Chris Thomas"),
        content: String::from("The Penguins are somehow the best team in the NHL again"),
    };

    println!("New article available {}", article.summarize());

    notify(&article);
}


