//use rust_book::ch17_async::*;
use message_queue::*;
use std::cell::RefCell;

// GlobalVars 

fn main() {
    
    // This default thing is pretty cool but I don't know if I did this
    let app = RefCell::new(MessageApp{..Default::default()});

    // Can add a message to any queue with a hashmap...but it's actually a raw quuee object. Programming is so sick lol
    // Actually, that's annoying, let's use a hashmap
    for i in 1..10 {
        let q_no = if i%2==1 {1} else {i};
        let m = Message{key: String::from(format!("{q_no}")), msg: String::from(format!("{i}th success?"))};
        app.borrow_mut().push(m);
    }
    // Can list all of them
    //app.borrow().list_message_queues();
    println!("{:#?}", app.borrow());
    // And thats it :P
    // Data Driven programming FTW!
}


