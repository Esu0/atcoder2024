use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: u32,
                }
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: u32,
                }
                if map[&x] == 1 {
                    map.remove(&x);
                } else {
                    *map.get_mut(&x).unwrap() -= 1;
                }
            }
            3 => {
                println!("{}", map.len());
            }
            _ => unreachable!(),
        }
    }
}
