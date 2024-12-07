use proconio::{input, marker};
use util::upper_bound;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: marker::Bytes,
    }
    let kth1 = {
        let mut v = Vec::with_capacity(n);
        for (i, &ci) in s.iter().enumerate() {
            if ci == b'1' {
                v.push(i);
            }
        }
        v
    };

    let kth2r = {
        let mut v = Vec::with_capacity(n);
        for (i, &ci) in s.iter().enumerate().rev() {
            if ci == b'2' {
                v.push(i);
            }
        }
        v
    };

    let count1 = {
        let mut v = Vec::with_capacity(n);
        let mut count = 0;
        for &ci in s.iter() {
            v.push(count);
            count += (ci == b'1') as usize;
        }
        v
    };

    let count2r = {
        let mut v = vec![0; n];
        let mut count = 0;
        for (i, &ci) in s.iter().enumerate().rev() {
            v[i] = count;
            count += (ci == b'2') as usize;
        }
        v
    };

    let slash = {
        let mut v = Vec::with_capacity(n + 1);
        v.push(0);
        for &ci in s.iter() {
            let count = *v.last().unwrap() + (ci == b'/') as usize;
            v.push(count);
        }
        v
    };
    // eprintln!("{:?}", kth1);
    // eprintln!("{:?}", kth2r);
    // eprintln!("{:?}", count1);
    // eprintln!("{:?}", count2r);
    // eprintln!("{:?}", slash);


    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let l = l - 1;
        let r = r - 1;
        let k = count1[l];
        let m = count2r[r];
        let ans = upper_bound(0..=r - l / 2, |x| {
            let i = k + x;
            let j = m + x;
            let nl = if i == 0 {
                l
            } else if i > kth1.len() {
                return false;
            } else {
                kth1[i - 1].max(l)
            };
            let nr = if j == 0 {
                r
            } else if j > kth2r.len() {
                return false;
            } else {
                kth2r[j - 1].min(r)
            };
            slash[nr + 1] as isize - slash[nl] as isize > 0
        });
        if ans == 0 {
            println!("0");
        } else {
            println!("{}", ans * 2 - 1);
        }
    }
}
