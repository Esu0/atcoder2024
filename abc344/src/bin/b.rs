use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    
    let mut v = Vec::new();
    for a in s.split_whitespace() {
        v.push(a.parse::<u32>().unwrap());
    }
    v.iter().rev().for_each(|&x| println!("{}", x));
}
