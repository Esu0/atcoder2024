use std::collections::{BTreeSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        q: usize
    }
    let mut id = 0;
    let mut set = BTreeSet::new();
    let mut arr = VecDeque::new();
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! { x: u64 }
                arr.push_back(x);
            }
            2 => {
                if let Some((x, _)) = set.pop_first() {
                    println!("{}", x);
                } else {
                    println!("{}", arr.pop_front().unwrap());
                }
            }
            3 => {
                arr.iter().for_each(|&x| {
                    set.insert((x, id));
                    id += 1;
                });
                arr.clear();
            }
            _ => unreachable!(),
        }
    }
}
