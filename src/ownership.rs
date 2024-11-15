// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn scope() {
    let s = String::from("something");

    {
        let s_inner = String::from("inner");
    }
}

fn copy() {
    // size known at compile time -> stack
    let x = 5;
    let y = x;
}

fn copy_string() {
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // size not known at compile time -> heap
    let s1 = String::from("hello");
    // this is called a move
    let s2 = s1;

    // println!("{s1}, world!");
}

fn function_ownership() {
    fn main() {
        let s = String::from("hello");
        takes_ownership(s);

        let x = 5;
        makes_copy(x);
    }

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    }
}

fn references() {
    // & is used for referencing
    // * is used for dereferencing
    fn main() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{s1}' is {len}.");
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}

fn reference_types() {
    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
}
