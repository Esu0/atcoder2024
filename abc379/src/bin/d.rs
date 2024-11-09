use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut offset = 0i64;
    let mut set = VecDeque::new();
    for _ in 0..q {
        input! {
            t: u8
        }
        match t {
            1 => {
                set.push_front(-offset);
            }
            2 => {
                input! {
                    t: u64,
                }
                offset += t as i64;
            }
            3 => {
                input! {
                    h: i64,
                }
                let h = h - offset;
                let mut count = 0;
                while let Some(&x) = set.back() {
                    if x >= h {
                        count += 1;
                        set.pop_back();
                    } else {
                        break;
                    }
                }
                println!("{}", count);
            }
            _ => unreachable!(),
        }
    }
}
