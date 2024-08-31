use proconio::input;
use util::algorithm;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        arr: [u64; n],
    }
    let ans = algorithm::upper_bound(0..=1_000_000_000u64, |x| {
        let mut add_count = 0;
        let mut sub_count = 0;
        for &ai in &arr {
            if ai > x {
                let diff = ai - x;
                sub_count += diff / b;
            } else {
                let diff = x - ai;
                add_count += (diff + a - 1) / a;
            }
        }
        add_count <= sub_count
    });
    println!("{}", ans - 1);
}
