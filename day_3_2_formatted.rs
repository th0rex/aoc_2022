fn main() {
    print!(
        "{}",
        /*r#"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"#*/
        include_str!("3")
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .fold(0, |a, x| {
                use std::collections::HashSet as H;
                use std::iter::FromIterator;
                let f = *H::<char>::from_iter(
                    H::<char>::from_iter(x[0].chars())
                        .intersection(&H::from_iter(x[1].chars()))
                        .cloned(),
                )
                .intersection(&H::from_iter(x[2].chars()))
                .next()
                .unwrap() as u8;
                a + if f > b'a' {
                    f - b'a' + 1
                } else {
                    f - b'A' + 27
                } as u64
            })
    );
}
