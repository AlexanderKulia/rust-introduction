#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got: {val}");
        }

        let plus_one_iter = v1.iter().map(|x| {
            println!("map1 {}", x);
            x + 1
        });
        let times_two_iter = plus_one_iter.map(|x| {
            println!("map2 {}", x);
            x * 2
        });
        let less_then_hundred_iter = times_two_iter.filter(|x| {
            println!("filter {}", x);
            *x < 100
        });
        let s: u32 = less_then_hundred_iter.sum();
        println!("{}", s);
    }

    #[test]
    fn reduce_example() {
        let reduced: i32 = (1..10).reduce(|acc, e| acc + e).unwrap();
        assert_eq!(reduced, 45);

        let folded: i32 = (1..10).fold(0, |acc, e| acc + e);
        assert_eq!(folded, 45);
    }
}
