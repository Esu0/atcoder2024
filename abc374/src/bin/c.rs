use proconio::input;

fn main() {
    input! {
        n: usize,
        k: [u64; n],
    }

    let ans = (0..(1u32 << n)).map(|bit| {
        let mut a = 0;
        let mut b = 0;
        for i in 0..n {
            if bit & (1 << i) == 0 {
                a += k[i];
            } else {
                b += k[i];
            }
        }
        a.max(b)
    }).min().unwrap();
    println!("{}", ans);
}
