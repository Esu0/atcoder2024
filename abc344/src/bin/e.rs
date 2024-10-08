use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        q: usize,
    }

    let mut ind = HashMap::new();
    let mut a = a.iter().enumerate().map(|(i, &x)| {
        ind.insert(x, i);
        let prev = if i == 0 { usize::MAX } else { i - 1 };
        let next = if i == n - 1 { usize::MAX } else { i + 1 };
        (x, prev, next)
    }).collect::<Vec<_>>();
    let mut head = 0;
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: u32,
                    y: u32,
                }
                let ind_x = ind[&x];
                let next = a[ind_x].2;
                let ind_y = a.len();
                ind.insert(y, ind_y);
                a[ind_x].2 = ind_y;
                if next != usize::MAX {
                    a[next].1 = ind_y;
                }
                a.push((y, ind_x, next));
            }
            2 => {
                input! {
                    x: u32,
                }
                let ind_x = ind.remove(&x).unwrap();
                let (_, prev, next) = a[ind_x];
                if prev == usize::MAX {
                    head = next;
                } else {
                    a[prev].2 = next;
                }
                if next != usize::MAX {
                    a[next].1 = prev;
                }
            }
            _ => unreachable!(),
        }
    }
    while head != usize::MAX {
        print!("{} ", a[head].0);
        head = a[head].2;
    }
    println!();
}
