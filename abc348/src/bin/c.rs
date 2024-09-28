use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(u32, u32); n],
    }
    let mut map = HashMap::new();
    for &(a, c) in &ac {
        map.entry(c).and_modify(|e| if *e > a { *e = a }).or_insert(a);
    }
    let ans = *map.values().max().unwrap();
    println!("{ans}");
}
