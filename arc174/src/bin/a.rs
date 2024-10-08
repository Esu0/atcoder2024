use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i64,
        a: [i64; n],
    }

    let mut s = std::iter::once(0).chain(a.iter().copied()).collect::<Vec<_>>();
    for i in 1..=n {
        s[i] += s[i - 1];
    }
}
