fn main() {
    // part 1: 14069
    // part 2: 12411
    println!(
        "{}",
        include_str!("2")
            /*        r#"A X
            A Y
            A Z
            B X
            B Y
            B Z
            C X
            C Y
            C Z"#*/
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

                // day 2:
                // X == 0
                // Y == 3
                // Z == 6
                //
                // (x[2]-b'X') * 3 for score of round
                //
                // 3 2 1 -- 3 1 2
                // 4 3 2 -- 1 2 3
                // 5 4 3 -- 2 3 1
                //
                // 0 2 1 -- 3 1 2
                // 1 0 2 -- 1 2 3
                // 2 1 0 -- 2 3 1
                // meh
                // what if we do + instead
                // 0 1 2 -- 3 1 2
                // 1 2 0 -- 1 2 3
                // 2 0 1 -- 2 3 1
                // wait we can shift this shit, if we +2 it
                // 2 0 1 -- 3 1 2
                // 0 1 2 -- 1 2 3
                // 1 2 0 -- 2 3 1
                // then we just + 1 it

                /*
                let z = (x[2] - b'X') as usize * 3;
                let d = (x[0] as usize + 26 - x[2] as usize) % 3;
                (d ^ (!(d >> 1) & 1)) + z
                 */

                ((x[0] + x[2] + 2) % 3 + 1 + (x[2] - b'X') * 3) as usize

                /*(match (x[0] as char, x[2] as char) {
                    ('A', 'X') => 3,
                    ('A', 'Y') => 1,
                    ('A', 'Z') => 2,
                    ('B', 'X') => 1,
                    ('B', 'Y') => 2,
                    ('B', 'Z') => 3,
                    ('C', 'X') => 2,
                    ('C', 'Y') => 3,
                    ('C', 'Z') => 1,
                    _ => unreachable!(),
                }) + (x[2] - b'X') as usize * 3*/

                // [3, 0, 6][(x[0] as usize + 26 - x[2] as usize) % 3] + (x[2] - 'W' as u8) as usize
                /*let d = (x[0] as usize + 26 - x[2] as usize) % 3;
                (d ^ (!(d >> 1) & 1)) * 3 + (x[2] - 'W' as u8) as usize*/

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
