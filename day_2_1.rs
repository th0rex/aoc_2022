fn main(){println!("{}",include_bytes!("2").chunks(4).map(|x|{let d=(x[0]as u64+26-x[2]as u64)%3;(d^(!(d>>1)&1))*3+(x[2]-87)as u64}).sum::<u64>());}
