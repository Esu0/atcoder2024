use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }
    let h1 = h[0];
    let ans = h[1..].iter().enumerate().find(|&(_, &hi)| hi > h1).map_or(-1, |(i, _)| (i + 2) as i32);
    println!("{}", ans);
}
