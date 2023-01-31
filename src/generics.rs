pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
pub struct Point_M<T, U> {
    pub x: T,
    pub y: U,
}

fn using_point() {
    let a = Point { x: 21, y: 36 };
    let b = Point_M { x: 32, y: 'f' };
}

//impl on all point generic types
//TODO why return a reference
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//impl only on Point<f32>

//TODO return a reference instead of copy
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//impl for certain traits
impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        //TODO when to use reference?
        if self.x > self.y {
            //accessing a property on a reference
            println!("larger number is x {}", self.x);
        }
    }
}

//mixed up
//The method creates a new Point instance with the x value from the self Point
//(of type T1) and the y value from the passed-in Point (of type U2).

impl<T, U> Point_M<T, U> {
    //TODO the self has no reference in this method
    pub fn mixup<T2, U2>(self, other: Point_M<T2, U2>) -> Point_M<T, U2> {
        Point_M {
            x: self.x,
            y: other.y,
        }
    }
}
