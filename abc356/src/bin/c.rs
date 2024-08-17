use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u32,
        t: [([usize], char); m],
    }
    let t = t.iter().map(|(a, r)| {
        let mut ai = 0u32;
        for &aij in a {
            ai |= 1 << (aij - 1);
        }
        (ai, *r)
    }).collect::<Vec<_>>();
    let mut ans = 0u32;
    for i in 0..(1 << n) {
        if t.iter().all(|&(ai, ri)| {
            let cnt = (i & ai).count_ones();
            ri == 'o' && cnt >= k || ri == 'x' && cnt < k
        }) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
