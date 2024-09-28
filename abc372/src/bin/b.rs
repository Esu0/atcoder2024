use proconio::input;

fn main() {
    input! {
        mut m: u32,
    }
    let mut ans = vec![];
    let mut i = 10;
    let mut a = 3u32.pow(10);
    while i >= 0 {
        while m >= a {
            m -= a;
            ans.push(i);
        }
        i -= 1;
        a /= 3;
    }
    println!("{}", ans.len());
    ans.iter().for_each(|&x| print!("{} ", x));
    println!();
}
