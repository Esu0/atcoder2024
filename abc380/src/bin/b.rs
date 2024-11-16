use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    let mut ans = vec![];
    let mut count = 0;
    for &ci in &s[1..] {
        if ci == b'-' {
            count += 1;
        } else {
            ans.push(count);
            count = 0;
        }
    }
    print!("{}", ans[0]);
    ans[1..].iter().for_each(|&a| print!(" {}", a));
    println!()
}
