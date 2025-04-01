// Structs, enums, and fns can have multiple generics
struct Point<T, U> {
    x: T,
    y: U
}

// Can impl on generics
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Can impl on select subsets
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// generic types in defs can be different than  struct types
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point{x: self.x, y: other.y}
    }
}



pub fn example() {
    let both_integer = Point { x: 5, y: 10};
    let mixed = Point { x: 5.0, y: "HAH"};
    println!("mixed.x = {}", mixed.x());
    let p3 = both_integer.mixup(mixed);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
