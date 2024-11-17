use std::collections::HashMap;

mod variables {
    fn main() {
        let x = 5;
        println!("The value of x is: {x}");
        // x = 6;
        // println!("The value of x is: {x}");

        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    }
}

mod data_types {
    use std::collections::HashMap;

    fn main() {
        // will fail without type annotation
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
}

mod functions {
    fn do_outer() {
        do_inner(5, 'h');
    }

    fn do_inner(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }

    fn main() {
        // Statements are instructions that perform some action and do not return a value.
        // Expressions evaluate to a resultant value.
        let y = 6; // statement

        let new_x = {
            let x = 3;
            x + 1
        }; // expression
    }

    fn multiply_by_two(value: i32) -> i32 {
        value * 2
    }
}

mod control_flow {
    fn basic_if() {
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }

    fn conditions_must_be_booleans() {
        let number = 3;

        // if number {
        //     println!("number was three");
        // }
    }

    fn else_if() {
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    fn ternary() {
        let condition = true;
        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }

    fn regular_loop() {
        loop {
            println!("again!");
        }
    }

    fn while_loop() {
        let mut number = 3;

        while number != 0 {
            println!("{number}!");

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }

    fn for_element_in_collection() {
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }
    }

    fn for_element_in_range() {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}
