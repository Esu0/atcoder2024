use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    let mut ans = 0;
    while a.iter().filter(|&&x| x > 0).count() > 1 {
        ans += 1;
        a.sort_unstable_by_key(|&x| Reverse(x));
        a[0] -= 1;
        a[1] -= 1;
    }
    println!("{ans}");
}
