use std::ops::{Add, Div, Mul, RangeBounds, Rem, Sub};

use proconio::input;

#[allow(dead_code)]
trait Integer:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Ord
    + Copy
{
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
}

impl Integer for i32 {
    const MIN: Self = i32::MIN;
    const MAX: Self = i32::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
}

impl Integer for u64 {
    const MIN: Self = u64::MIN;
    const MAX: Self = u64::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
}

/// `range`が`l..r`で、返り値を`i`とすると、
/// `(l..i).contains(j)`となる`j`において、`f(j)`が`true`となり、
/// `(i..r).contains(j)`となる`j`において、`f(j)`が`false`となる。
fn partition_point<T: Integer>(range: impl RangeBounds<T>, mut f: impl FnMut(T) -> bool) -> T {
    let mut l = match range.start_bound() {
        std::ops::Bound::Included(&l) => l,
        std::ops::Bound::Excluded(&l) => l + T::ONE,
        std::ops::Bound::Unbounded => T::MIN,
    };
    let mut r = match range.end_bound() {
        std::ops::Bound::Included(&r) => r + T::ONE,
        std::ops::Bound::Excluded(&r) => r,
        std::ops::Bound::Unbounded => T::MAX,
    };
    while r - l > T::ONE {
        let m = l + (r - l) / T::TWO;
        if f(m) {
            l = m + T::ONE;
        } else {
            r = m;
        }
    }
    if f(l) {
        l + T::ONE
    } else {
        l
    }
}

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
    }
    let sum = a.iter().sum::<u64>();
    if sum <= m {
        println!("infinite");
    } else {
        let ans = partition_point(0..=sum, |x| {
            let sum = a.iter().map(|&a| std::cmp::min(a, x)).sum::<u64>();
            sum <= m
        });
        println!("{}", ans - 1);
    }
}
