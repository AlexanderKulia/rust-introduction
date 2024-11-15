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

pub fn multiply_by_two(value: i32) -> i32 {
    value * 2
}
