# Rust introduction

## What is Rust

Rust is a systems programming language known for its memory safety, performance and concurrency

- Compiled
- Statically typed
- Cross-platform
- Does not have garbage collection, but has "borrow checker"

## Code example

[Source](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Resources

- The Rust Programming Language https://doc.rust-lang.org/book/
- Exercises https://github.com/rust-lang/rustlings
