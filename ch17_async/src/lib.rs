// Part 1 - these require two links as cmd line args
use trpl::{Either, Html};
use std::{thread, pin::Pin, time::Duration};
// Async keywaord does some pretty complicated compilation tricks to return a Future opbject under
// the hood
//
// Overall, we are mostly using the trpl runtime here
pub async fn page_title(url: &str) -> (&str, Option<String>) {
    // Could also chain as trpl::get(url).await.response.text().await
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

pub fn basic_futures() {
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

// Some thread like future examinations
pub fn concurrency_example_17_2a() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(5)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(5)).await;
        }
        // Without this bit, we end the task when this above loop is done.
        // await returns a Result type so we can unwrap!
        // This is like join from the threads work (ch 16)
        handle.await.unwrap()
    });

    // Message parsing example using trpl channel
    // This is an async version of mpsc *multiple pproducer, single consumer)
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // Create separate async blocks to allow to run async
        // We must move the tx_future variable to allow the borow checker to drop after this last
        // message is sent
        let tx2 = tx.clone();
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(5)).await;
            }
        };


        // The while let loop lets us wait in a loop for awaits
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("Got: {value}");
            }
        };

        // Clone works on tx_futs cause channel is multiple producers
        let tx2_fut = async move {
            let vals = vec![
                String::from("also"),
                String::from("these"),
                String::from("messages"),
                String::from("exist"),
            ];

            for val in vals {
                tx2.send(val).unwrap();
                trpl::sleep(Duration::from_millis(10)).await;
            }
        };

        // This join(XX, YY).await is annoying... but there is a nice macro!
        // trpl::join!(tx_fut, tx2_fut, rx_fut);

        // This requires us to know the amount ahead of time tho....
        // So we use join_all
        /* I'm not entirely clear why the bottom doesn't work. Something about async making
         * a unique enum for each async block?
         *
         * let futures = vec![tx_fut, tx2_fut, rx_fut];
         * trpl::join_all(futures).await;
         *
         * So because of that we need to use something weird called trait objects!
         */
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx_fut), Box::pin(tx2_fut), Box::pin(rx_fut)];

        trpl::join_all(futures).await;
        // Honestly this is so complex, so see rust book for deets https://doc.rust-lang.org/stable/book/ch17-03-more-futures.html
        // It also doesn't talk about pin lol
        // Also join_all must be same type while not necessary for trpl::join!
    });
}

// Note thread::sleep blocks the thread
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}ms");
}
// Some examples of yielding to the runtime
pub fn concurrency_example_17_2b() {
    trpl::run(async {
        let a = async {
            println!("a started");
            // We can hand back control of the runtime with yield_now
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("a is finished");
        };

        let b = async {
            println!("b started");
            slow("b", 70);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("b is finished");
        };

        trpl::race(a, b).await;

    })
}


// Implement a custom timeout function!
pub fn concurrency_example_17_2c() {

    async fn timeout<F: Future>(
        future_to_try: F,
        max_time: Duration,
    ) -> Result<F::Output, Duration> {
        match trpl::race(future_to_try, trpl::sleep(max_time)).await {
            Either::Left(output) => Ok(output),
            Either::Right(_) => Err(max_time)
        }
    }


    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(10000)).await;
            "I finished!!!"
        };

        match timeout(slow, Duration::from_millis(1000)).await {
            Ok(message) => println!("Succeeded with {message}"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    })
}
