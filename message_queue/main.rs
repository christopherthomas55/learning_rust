use rust_book::ch17_async::*;
use super;

use std::collections::HashMap;

// GlobalVars 

Q Queue<
Hm HashMap{}
Q.hashmap = HM;

fn main() {
    //let queues = vec!(queue);

    // Can add a message to any queue with a hashmap...but it's actually a raw quuee object. Programming is so sick lol
    // Actually, that's annoying, let's use a hashmap
    Q.push(Message{"test", "success?"});
    // Actually can't do hashmap cause need ordeting hm.insert("test", "success?"); 

    // Can list all of them
    // list_message_queues(queues);
    /* Btw, this function only does this
     *
    for q in queues {
        for content in q {
            print!("{content}");
        }
    }; 
     *
     */
    print!("Q");
    print!("HM")


    // And thats it :P
    // Data Driven programming FTW!
}


