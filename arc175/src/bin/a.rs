use proconio::{input, marker};
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
        s: marker::Bytes,
    }
    for pi in &mut p {
        *pi -= 1;
    }
    let mut h = vec![false; n];
    let mut ans = MInt::new(0);
    let mut ans0 = MInt::new(1);
    // 最初に行動する人が左利き
    for &pi in &p {
        if h[(pi + 1) % n] {
            // 右隣の人がすでにスプーンをとっている
            if s[pi] == b'?' {
                ans0 *= ModInt::new(2);
            }
        } else if s[pi] == b'R' {
            ans0 *= ModInt::new(0);
            break;
        }
        h[pi] = true;
    }
    ans += ans0;
    ans0 = MInt::new(1);
    h.fill(false);
    // 最初に行動する人が右利き
    for &pi in &p {
        if h[(pi + n - 1) % n] {
            // 左隣の人がすでにスプーンをとっている
            if s[pi] == b'?' {
                ans0 *= ModInt::new(2);
            }
        } else if s[pi] == b'L' {
            ans0 *= ModInt::new(0);
            break;
        }
        h[pi] = true;
    }
    ans += ans0;
    println!("{}", ans);
}
