use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32; n],
    }

    let mut set = HashMap::new();
    let mut ans = 0;
    let mut r = 0;
    for l in 0..n {
        while r < n && set.len() <= k {
            ans = ans.max(r - l);
            set.entry(a[r]).and_modify(|x| *x += 1).or_insert(1);
            r += 1;
        }
        if set.len() <= k {
            ans = ans.max(r - l);
        }
        let x = set.get_mut(&a[l]).unwrap();
        *x -= 1;
        if *x == 0 {
            set.remove(&a[l]);
        }
    }
    println!("{}", ans);
}
