use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let n = 99999 + n;
    let s = n.to_string().into_bytes();
    let t = [0,0,1,2,3,3,4,5,4].iter().map(|&x| s[x]).collect::<Vec<_>>();
    println!("{}", std::str::from_utf8(&t).unwrap());
}
