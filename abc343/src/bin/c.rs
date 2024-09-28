use proconio::input;

fn is_palindrome_number(n: u64) -> bool {
    let s = n.to_string();
    let rev = String::from_utf8(s.bytes().rev().collect::<Vec<_>>()).unwrap();
    s == rev
}

fn main() {
    input! {
        n: u64,
    }
    let mut cubic = vec![];
    let mut i = 1u64;
    while i.checked_mul(i).and_then(|x| x.checked_mul(i)).is_some_and(|x| x <= n) {
        cubic.push(i * i * i);
        i += 1;
    }
    while let Some(n) = cubic.pop() {
        if is_palindrome_number(n) {
            println!("{n}");
            return;
        }
    }
}
