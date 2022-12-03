fn main() {
    print!(
        "{}",
        /*        r#"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"#*/
        include_str!("3")
            .lines()
            .map(|x| {
                let mut s = std::collections::HashSet::<u8>::new();
                let x = x.as_bytes();
                s.extend(&x[..x.len() / 2]);
                for &x in &x[x.len() / 2..] {
                    if s.remove(&x) {
                        return if x > b'a' {
                            x - b'a' + 1
                        } else {
                            x - b'A' + 27
                        } as u64;
                    }
                }
                0
            })
            .sum::<u64>()
    );
}
