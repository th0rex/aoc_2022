use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    let mut c = std::path::PathBuf::new();
    c.push("/");
    let mut v = c.clone();
    for l in INPUT.lines() {
        match l.as_bytes() {
            b"$ cd /" => v=c.clone(),
            b"$ ls" => {},
            b"$ cd .." => {v.pop();},
            [b'$', _, b'c', b'd', _, x@..] => v.push(std::str::from_utf8(x).unwrap()),
            x => if let Some(Ok(s)) = x.split(|x|x==&b' ').next().map(|x| std::str::from_utf8(x).unwrap().parse::<u64>()) {
                let mut t = std::ffi::OsString::new();
                for (i,x) in v.iter().enumerate() {
                    t.push(x);
                    *m.entry(t.clone()).or_insert(0) += s;
                    if i>0{t.push("/");}
                }
            }
        }
    }

    //print!("{}", m.values().filter(|x| x < &&100001).sum::<u64>());

    let c = m[&{let mut c = std::ffi::OsString::new(); c.push("/"); c}];
    print!("{}", m.values().filter(|&&x| x >= c - 40000000).min().unwrap());
}
