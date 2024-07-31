use proconio::input;

const MODULO: u64 = 998244353;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut sum10 = vec![0; n];
    let mut sums = vec![0; n];
    let mut sum1 = 0u64;
    let mut sum2 = 0u64;
    for (i, &ai) in a.iter().enumerate().rev() {
        sum1 = (sum1 + ai) % MODULO;
        sums[i] = sum1;
        sum2 = (sum2 + 10u64.pow(ai.ilog10() + 1)) % MODULO;
        sum10[i] = sum2;
    }

    eprintln!("{:?}", sum10);
    let mut sum = 0;
    for (i, &ai) in a[..n - 1].iter().enumerate() {
        sum = (sum + sums[i + 1] + sum10[i + 1] * ai) % MODULO;
    }
    println!("{}", sum);
}
