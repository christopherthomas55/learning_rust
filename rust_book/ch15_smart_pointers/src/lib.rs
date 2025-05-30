pub fn refcell_example() {
    // This is some unsafe rust stuff. I hope I don't have to use this often
    // Essentially, no longer enforces borrow rules until runtime
    // Recap rules ( - one mutable reference or multiple immutables - always valid references)
    // This is halting problem type stuff
    // once again this is only for single threaded case right now
    // 
    // Long story short, this allows us to mutate an immutable value while appearing mutable to
    // other code
    // Useful for mocks
    /* Recap - this fails since x is immutable
    let x = 5;
    let y = &mut x; */
    mutable_rc_example();
    mem_leak_example();
    weak_ref_example();
}

// We typically find this useful in tests. See below, where we want to mock a messenger
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: Over quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent! You are at 90% quota usage");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning! At 75% quota usage");
        }
    }
}


mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // If we use just a vec string we get an error since we expect usage of MockMessenger to be
        // immutable refs in non test code
        // RefCell to the rescue!
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // RefCell does runtime borrow checks so the following will break 
            /*
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();
            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));*/
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn test_75_over_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

    }
}

// We can also combine rc and Refcells to have multiple owners of mutable data if you're feeling
// spicy

#[derive(Debug)]
enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil
}

use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn mutable_rc_example() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

// But we can also have memory leaks
use self::LeakList::{LCons, LNil};
#[derive(Debug)]
enum LeakList {
    LCons(i32, RefCell<Rc<LeakList>>),
    LNil,
}

impl LeakList {
    fn tail(&self) -> Option<&RefCell<Rc<LeakList>>> {
        match self {
            LCons(_, item) => Some(item),
            LNil => None,
        }
    }
}

fn mem_leak_example() {
    let a = Rc::new(LCons(5, RefCell::new(Rc::new(LNil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(LCons(20, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count after changing a= {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));

    // This will overflow the stack trying to print circular references
    // println!("a next item = {:?}", a.tail());
}

use std::rc::Weak;
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn weak_ref_example() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    // Adding a weak reference with Rc:downgrade above prevents an infinite reference here
    // Weak references don't prevent Rc pointers from dropping either
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
