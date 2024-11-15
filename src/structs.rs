fn regular_structs() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
}

fn unit_like_structs() {
    struct AlwaysEqual;

    fn main() {
        let subject = AlwaysEqual;
    }
}

fn struct_methods() {
    // also called associated functions
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
}

fn generic_structs() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    fn main() {
        let integer = Point { x: 5, y: 10.0 };
        let float = Point { x: 1, y: 4.0 };
    }
}

fn generics_in_struct_methods() {
    // https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn main() {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }
}

fn traits() {
    pub trait Summary {
        fn summarize(&self) -> String; // this can be a default implementation
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // use traits as parameters. this is syntax sugar for the version below
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify_with_boundary<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // multiple bounds
    // pub fn notify(item: &(impl Summary + Display)) {}
    // pub fn notify<T: Summary + Display>(item: &T) {}

    // alternative syntax
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {}

    // implement trait for any type that implements another trait
    // impl<T: Display> ToString for T {}
}
