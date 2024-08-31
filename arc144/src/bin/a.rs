use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", 2 * n);
    let mut n = n;
    let mut ans = vec![];
    while n >= 4 {
        ans.push(b'4');
        n -= 4;
    }
    if n != 0 {
        ans.push(b'0' + n as u8);
    }
    ans.reverse();
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
