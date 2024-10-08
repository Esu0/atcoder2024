use proconio::{input, marker};
use util::ModInt;
type MInt = ModInt<998244353>;

fn main() {
    input! {
        _n: usize,
        mut a: marker::Bytes,
        mut b: marker::Bytes,
    }
    for (ai, bi) in a.iter_mut().zip(&mut b) {
        *ai -= b'0';
        *bi -= b'0';
        if *ai > *bi {
            std::mem::swap(ai, bi);
        }
    }
    let mut a1 = MInt::new(0);
    for &ai in a.iter() {
        a1 *= MInt::new(10);
        a1 += MInt::new(ai as _);
    }
    let mut b1 = MInt::new(0);
    for &bi in b.iter() {
        b1 *= MInt::new(10);
        b1 += MInt::new(bi as _);
    }
    println!("{}", a1 * b1);
}
