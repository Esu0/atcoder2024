use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let mut ans = 0;
    for c in -1000..=1000 {
        let mut a = [a, b, c];
        a.sort_unstable();
        if a[0] - a[1] == a[1] - a[2] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
