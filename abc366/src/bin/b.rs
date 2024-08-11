use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: [marker::Bytes; n],
    }
    let mut ans = vec![];
    let mut i = 0;
    loop {
        let mut s = (0..n)
            .rev()
            .map(|j| s[j].get(i).copied().unwrap_or(b'*'))
            .collect::<Vec<_>>();
        while s.last() == Some(&b'*') {
            s.pop();
        }
        if s.is_empty() {
            break;
        }
        ans.push(s);
        i += 1;
    }
    ans.iter()
        .for_each(|s| println!("{}", std::str::from_utf8(s).unwrap()));
}
