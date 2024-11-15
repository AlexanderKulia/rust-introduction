use std::collections::HashMap;

fn main() {
    // will fail
    let guess: i32 = "42".parse().expect("Not a number!");

    // integers
    // length signed unsigned
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize
    let a: u8 = 1;

    // floats
    // f32 f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // boolean
    // true false
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character
    // char
    let x: char = 'c';

    // string slice
    // just a pointer to sequence of characters
    let my_string_slice = "something";

    // string
    // struct that stores a pointer to a string slice, length and capacity
    let my_string = String::from("something");
    let slice_from_string = &my_string[0..2];

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array
    // every element must be the same type
    // fixed length
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // in type annotation [type, length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // in value [value, length] for create [3, 3, 3, 3, 3]
    let a = [3; 5];

    // accesing array elements
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // vectors
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let third = v2.get(2);

    // hashmaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    scores.entry(String::from("Yellow")).or_insert(50);
}
