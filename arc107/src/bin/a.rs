use proconio::input;
use util::ModInt;
type MInt = ModInt<998244353>;
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let a = MInt::new(a);
    let b = MInt::new(b);
    let c = MInt::new(c);
    let inv = MInt::new(8).inv();
    let one = MInt::new(1);
    let ans = a * (a + one) * b * (b + one) * c * (c + one) * inv;
    println!("{}", ans);
}
