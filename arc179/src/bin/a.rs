use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        mut a: [i32; n],
    }
    if k > 0 {
        a.sort_unstable();
        println!("Yes");
        a.iter().for_each(|&ai| print!("{} ", ai));
        println!();
    } else {
        let sum = a.iter().fold(0i64, |acc, &ai| acc + ai as i64);
        if sum >= k as i64 {
            println!("Yes");
            a.sort_unstable_by_key(|&ai| Reverse(ai));
            a.iter().for_each(|&ai| print!("{} ", ai));
        } else {
            println!("No");
        }
    }
}
