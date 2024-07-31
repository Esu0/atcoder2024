use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    let mut a_sum = Vec::with_capacity(n);
    let mut sum = a[..m].iter().sum::<i64>();
    a_sum.push(sum);
    for (&ai, aj) in a.iter().zip(&a[m..]) {
        sum += aj - ai;
        a_sum.push(sum);
    }

    let mut b_sum = Vec::with_capacity(n - m + 1);
    let mut first_sum = a[..m]
        .iter()
        .enumerate()
        .map(|(i, &ai)| (i + 1) as i64 * ai)
        .sum::<i64>();
    b_sum.push(first_sum);
    for (&a_sum_i, &aj) in a_sum.iter().zip(&a[m..]) {
        first_sum += aj * m as i64 - a_sum_i;
        b_sum.push(first_sum);
    }
    println!("{}", b_sum.iter().copied().max().unwrap());
}
