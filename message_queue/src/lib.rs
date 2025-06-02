use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    pub key:        String,
    pub msg:        String,
}
impl Clone for Message {
    fn clone(&self) -> Message {
        Message{
            key: self.key.clone(),
            msg: self.msg.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Queue { 
    pub messages:   Vec<Message>,
    pub head:       Option<usize>, // Allows us to have a head eventually
}


#[derive(Debug)]
pub struct MessageApp {
    pub hm:     HashMap<String, Rc<RefCell<Queue>>>, // This is so complicated. Wanted a mutable Queue in the Hashmap
    pub num_queues: u64
}

// I really fought the compiler here. May be a simpler way to use this
impl Default for MessageApp {
    fn default() -> MessageApp {
        let hm: HashMap<String, Rc<RefCell<Queue>>> = HashMap::new();
        MessageApp {
            hm:             hm,
            num_queues:     0,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
const MAX_QUEUES: u64 = 10_000;
// Can I simplify this lifetime stuff? OMG - this feels wrong
// TODO - this clones message....need a better way
impl MessageApp {
    pub fn push (&mut self, m: Message){
        if let Some(q) = self.hm.get_mut(&m.key) {
            q.borrow_mut().messages.push(m);
        } else {
            if self.num_queues > MAX_QUEUES {panic!()}

            let q = Rc::new(RefCell::new(Queue{messages: vec!(), head: None}));
            self.hm.insert(m.key.clone(), q.clone());
            q.borrow_mut().messages.push(m);

            self.num_queues += 1;
        }

    }
    pub fn read_from_key(&self, key: &String, head: Option<usize>) -> String { // TODO Eventually add to_delete: bool)
        let q = self.hm.get(key);
        // We probably don't need to clone here
        match head {
            None => {
                q.unwrap().borrow().messages.get(0).unwrap().msg.clone()
            },
            Some(head) => {
                q.unwrap().borrow().messages.get(head).unwrap().msg.clone()
            }
        }
    }

    pub fn list_message_queues(&self){
        for (key, queue) in &self.hm {
            println!("{key:?}: {queue:#?}");
        }
    }
}
