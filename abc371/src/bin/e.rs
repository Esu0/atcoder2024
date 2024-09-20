use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut pos = vec![vec![]; n];
    for i in 0..n {
        pos[a[i] - 1].push(i);
    }

    let mut ans = 0;
    for x in &pos {
        let mut prev = n;
        for &p in x.iter().rev() {
            ans += (prev - p) * (p + 1);
            prev = p;
        }
    }
    println!("{}", ans);
}
