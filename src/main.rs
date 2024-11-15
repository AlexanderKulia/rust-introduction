use crate::basics::functions::multiply_by_two;

pub mod basics;
pub mod concurrency;
pub mod enums;
pub mod iterators;
pub mod lifetimes;
pub mod ownership;
pub mod structs;
pub mod tests;

fn main() {
    let res = multiply_by_two(2);
    println!("{}", res);
}
