// The first type of pointer-like type is Box
// It puts a value on the heap
//
// Box puts data on the heap and
// allows us to have recursive data types (linked lists, cons list, etc)
// I knew lisp would be useful one of these days!
// We use cons lists (1 (2 (3))) to demosntrate box

enum List <T> {
    // Remember that Cons here is an enum variant
    Cons(T,  Box<List<T>>),
    Nil
}

use self::List::{Cons, Nil};

fn box_example() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Let's build a box to see what dereferencing looks like
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// Right now, this struct + new trait will put a box on the heap
// To reference it we need to add a dereference trait below
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    // Ignore this associated type syntax for now, just think of it as weird generic type syntax
    type Target = T;

    // Notice we return a reference here...to play nice with borrow checker
    // Rust actually turns *y into *(y.deref())
    fn deref(&self) -> &Self::Target {
        // This is a bit weird, just returns first value in a tuple structure and MyBox is a one
        // tuple of type T
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn deref_trait_example() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("y is a pointer even though debug shows its val {:?}", y);

    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("y is a pointer even though debug shows its val {:?}", y);

    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref coercion also works - it essentially chains derefs on structs that have the deref
    // trait to simplify code
    // eg
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // without deref coercion this looks like
    // hello(&(*m)[..]);
    //
    // This laso works with mut &m, but must have the DerefMut trait
    // We will deref from &T -> &U, &mut T -> &mut U, and &mut T -> &U (notice mut change)
}

// Drop trait example time!!
struct CustomSmartPointer {
    data: String,
}

// Doing this part for fun
impl Deref for CustomSmartPointer {
    type Target = String;
    fn deref(&self) -> &String {
        // This is a bit weird, just returns first value in a tuple structure and MyBox is a one
        // tuple of type T
        &self.data
    }
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_example() {
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    hello(&c);
    // To drop values early, must use std::mem::drop
    drop(c);
    let c = CustomSmartPointer {
        data: String::from("my stuff 2")
    };
    drop(c);
    let c = CustomSmartPointer {
        data: String::from("my stuff 3")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
    println!("CustomSmartPointers created");
}

enum RCList<T> {
    RCCons(T, Rc<RCList<T>>),
    RCNil
}

use self::RCList::{RCCons, RCNil};
use std::rc::Rc;

fn rc_example() {
    // Reference counter
    // Lets us allocate to heap when we don't know at compile time which part will finish with
    // data last
    // We use the cons list again to demonstrate this
    /* This cannot compile because of ownership rules
    let a = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let b = Cons(3, Box::new(a));
    let c = Cons(3, Box::new(a));*/
    // But rc let's us clone references and track 
    let a = Rc::new(RCCons(1, Rc::new(RCCons(2, Rc::new(RCCons(3, Rc::new(RCNil)))))));
    // We don't use a.clone() here since we don't need a deep copy
    println!("count after creating a of refs = {}", Rc::strong_count(&a));
    let b = RCCons(3, Rc::clone(&a));
    println!("count after creating b of refs = {}", Rc::strong_count(&a));
    {
        let c = RCCons(3, Rc::clone(&a));
        println!("count after creating c of refs = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope of refs = {}", Rc::strong_count(&a));

    // Not that rc is immutable only....
}


use ch15_smart_pointers::refcell_example;
fn main() {
    box_example();
    deref_trait_example();
    drop_example();
    rc_example();
    refcell_example();
}
