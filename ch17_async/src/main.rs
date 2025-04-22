use trpl::{Either, Html};

// Async keywaord does some pretty complicated compilation tricks to return a Future opbject under
// the hood
async fn page_title(url: &str) -> (&str, Option<String>) {
    // Could also chain as trpl::get(url).await.response.text().await
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    basic_futures();
}

fn basic_futures() {
    let args: Vec<String> = std::env::args().collect();
    // Trpl runtime handles all the async stuff
    trpl::run( async {
        // Remember the await means these are futures
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);


        let (url, maybe_title) = 
            // select is typically used
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        println!("{url} returned first!");
        match maybe_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title")
        }
    })
}
