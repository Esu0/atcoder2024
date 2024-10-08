use proconio::input;
use util::algorithm::upper_bound;

fn main() {
    input! {
        n: usize,
        x: u32,
        apbq: [(u32, u32, u32, u32); n],
    }
    let ans = upper_bound(0u64..2000000000, |w| {
        apbq.iter().map(|&(ai, pi, bi, qi)| {
            let ai = ai as u64;
            let bi = bi as u64;
            let pi = pi as u64;
            let qi = qi as u64;
            let mut cost = u64::MAX;
            for x in 0..bi {
                let y = (w.saturating_sub(ai * x) + bi - 1) / bi;
                cost = cost.min(x * pi + y * qi);
            }
            for y in 0..ai {
                let x = (w.saturating_sub(bi * y) + ai - 1) / ai;
                cost = cost.min(x * pi + y * qi);
            }
            cost
        }).sum::<u64>() <= x as u64
    }) - 1;
    println!("{}", ans);
}
