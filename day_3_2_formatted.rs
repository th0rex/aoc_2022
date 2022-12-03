use std::{collections::HashSet as H, iter::FromIterator};
fn main() {
    print!(
        "{}",
        include_str!("3")
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .fold(0, |a, x| {
                let f = *H::<_>::from_iter(
                    H::<_>::from_iter(x[0].chars())
                        .intersection(&H::from_iter(x[1].chars()))
                        .cloned(),
                )
                .intersection(&H::from_iter(x[2].chars()))
                .next()
                .unwrap() as u64;
                a + if f > 97 { f - 96 } else { f - 38 }
            })
    );
}
