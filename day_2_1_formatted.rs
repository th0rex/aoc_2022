fn main() {
    // 14069
    println!(
        "{}",
        include_str!("2")
            .lines()
            .map(|x| {
                let x = x.as_bytes();
                // 3 2 1 -- 3 6 0
                // 4 3 2 -- 0 3 6
                // 5 4 3 -- 6 0 3
                //
                // 0 2 1 -- 3 6 0
                // 1 0 2 -- 0 3 6
                // 2 1 0 -- 6 0 3
                //
                // 0 -> 3
                // 1 -> 0
                // 2 -> 6
                //
                // want
                // 0 -> 0
                // 1 -> 3
                // 2 -> 6
                // so we can * 3
                //
                // flip 0 bit but only if number is not 2
                // -> (x ^ (!(x >> 1) & 1))
                //
                // x % 26 % 3 == x % 3

                // [3, 0, 6][(x[0] as usize + 26 - x[2] as usize) % 3] + (x[2] - 'W' as u8) as usize
                let d = (x[0] as usize + 26 - x[2] as usize) % 3;
                (d ^ (!(d >> 1) & 1)) * 3 + (x[2] - 'W' as u8) as usize
                /*(match (x[0] as char, x[2] as char) {
                    ('A', 'X') => 3,
                    ('A', 'Y') => 6,
                    ('A', 'Z') => 0,
                    ('B', 'X') => 0,
                    ('B', 'Y') => 3,
                    ('B', 'Z') => 6,
                    ('C', 'X') => 6,
                    ('C', 'Y') => 0,
                    ('C', 'Z') => 3,
                    _ => unreachable!(),
                }) + (x[2] - 'W' as u8) as usize*/
            })
            .sum::<usize>()
    );
}
