fn main(){println!("{}",include_bytes!("2").chunks(4).map(|x|((x[0]+x[2]+2)%3+1+(x[2]-88)*3)as u64).sum::<u64>());}
