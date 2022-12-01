fn main() {
    println!(
        "{}",
        include_str!("inputs/day_1_1")
            .lines()
            .fold((0, 0), |(m, c), l| if l == "" {
                (std::cmp::max(m, c), 0)
            } else {
                (m, c + l.parse::<usize>().unwrap())
            })
            .0
    );
}
