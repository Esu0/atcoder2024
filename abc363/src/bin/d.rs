use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    if n == 1 {
        println!("0");
        return;
    }
    let mut k = 2u64;
    while let Some(next_k) = k.checked_mul(10) {
        if next_k <= n {
            k = next_k;
        } else {
            break;
        }
    }
    let mut a = n - k;
    if a >= (k / 2) * 9 {
        a -= (k / 2) * 9;
        a += k / 2;
        print!("{}", a);
        let mut s = a.to_string().into_bytes();
        s.reverse();
        println!("{}", std::str::from_utf8(&s).unwrap());
    } else {
        a += k / 2;
        print!("{}", a);
        let mut s = a.to_string().into_bytes();
        s.reverse();
        println!("{}", std::str::from_utf8(&s[1..]).unwrap());
    }
}
