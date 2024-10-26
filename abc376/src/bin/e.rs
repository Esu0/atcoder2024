use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    let mut ab = vec![];
    for _ in 0..t {
        ab.clear();
        input! {
            n: usize,
            k: usize,
            a: [u64; n],
            b: [u64; n],
        }
        for (&ai, &bi) in a.iter().zip(&b) {
            ab.push((ai, bi));
        }
        ab.sort_unstable_by_key(|&(ai, _)| ai);

        let mut sum = ab[..k].iter().map(|&(_, bi)| bi).sum::<u64>();
        let mut queue = ab[..k].iter().map(|&(_, bi)| bi).collect::<BinaryHeap<_>>();
        let mut ans = sum * ab[k - 1].0;
        for &(ai, bi) in ab[k..].iter() {
            queue.push(bi);
            sum += bi;
            sum -= queue.pop().unwrap();
            ans = ans.min(sum * ai);
        }
        println!("{}", ans);
    }
}
