use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    key:        String,
    msg:        String,
}

#[derive(Debug)]
struct Queue <'a>{ 
    messages:   Vec<&'a Message>,
}


/* I have no idea how this part of the language works rip
 */
// This probably needs a lifetime?
// No this needs a refcell or ref....ugh. Later
const MAX_QUEUES: u16 = 100_000;
pub fn push <'a>(m: &'a Message, hm: &mut'a  HashMap<&String, &Queue>){
    if let Some(q) = hm.get(&m.key) {
        q.messages.push(&mut m);
        print!("To {q:?} appended {m:?}");
    } else {
        let mut q = Queue{messages: vec!()} ;
        hm.insert(&mut m.key, &mut q);
        q.messages.push(m);
        print!("Started new queue {q:?}");
    }

}

pub fn list_message_queues(hm: HashMap<&String, Queue>){
    for (_, queue) in &hm {
        println!("{queue:?}");
    }
}


