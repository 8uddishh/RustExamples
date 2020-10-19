use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::ops::Add;
use std::ops::Mul;

// trait MathOperatable<T>
// where
//     T: Mul + Add,
// {
//     fn Add(&lhs: &T, &rhs: &T) -> T {
//         lhs + rhs
//     }
// }

// #[derive(Eq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T: PartialEq> PartialEq for Point<T> {
//     fn eq(&self, other: &Point<T>) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

// impl<T> Ord for Point<T>
// where
//     T: Mul + Add,
// {
//     fn cmp(&self, other: &Point<T>) -> Ordering {
//         ((self.x * self.x) + (self.x * self.x)).cmp((other.x * other.x) + (other.x * other.x))
//     }
// }

// impl<T: PartialEq> PartialOrd for Point<T> {
//     fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);

    // println!("Largest {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest(&number_list);

    // println!("Largest {}", result);
}
