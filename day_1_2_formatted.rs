fn main() {
    println!(
        "{}",
        include_str!("inputs/day_1_1")
            .lines()
            .fold(([0; 3], 0), |(mut m, c), l| if l == "" {
                let x = m.iter_mut().min().unwrap();
                if c > *x {
                    *x = c;
                }
                (m, 0)
            } else {
                (m, c + l.parse::<usize>().unwrap())
            })
            .0
            .iter()
            .sum::<usize>()
    );
}
