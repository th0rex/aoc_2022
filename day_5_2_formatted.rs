fn main() {
    print!(
        "{}",
        r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
            //include_str!("5")
            .lines()
            .fold(vec![Vec::<char>::new(); 99], |mut a, l| {
                match l.as_bytes() {
                    b"" => {
                        for x in &mut a {
                            x.reverse();
                        }
                    }
                    [_, b'1', ..] => {}
                    [b'm', ..] => {
                        let x = l
                            .split(' ')
                            .skip(1)
                            .step_by(2)
                            .map(|x| x.parse().unwrap())
                            .collect::<Vec<usize>>();
                        let b = &mut a as *mut Vec<Vec<_>>;
                        let y = &mut a[x[1] - 1];
                        let z = unsafe { &mut (*(b))[x[2] - 1] };
                        z.extend(y.drain(y.len() - x[0]..));
                    }
                    l => {
                        for i in (1..l.len()).step_by(4) {
                            let x = l[i] as _;
                            if x != ' ' {
                                a[i / 4].push(x);
                            }
                        }
                    }
                }
                a
            })
            .iter()
            .filter_map(|x| x.last())
            .collect::<String>()
    );
}
