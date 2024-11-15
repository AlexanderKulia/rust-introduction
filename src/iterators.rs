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
        let s: u32 = times_two_iter.sum();
        println!("{}", s);
    }
}
