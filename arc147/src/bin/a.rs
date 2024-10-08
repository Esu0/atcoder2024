use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }
    a.sort_unstable();
    let mut a = VecDeque::from(a);
    let mut ans = 0;
    while a.len() > 1 {
        let ai = a.pop_back().unwrap();
        let aj = a.front().copied().unwrap();
        let new = ai % aj;
        if new != 0 {
            a.push_front(new);
        }
        ans += 1;
    }
    println!("{}", ans);
}
