use core::f64;

use proconio::input;

fn permutation(n: usize, i: usize, buf: &mut [u8], f: &mut impl FnMut(&[u8])) {
    if i == n {
        f(buf);
        return;
    }

    for j in i..n {
        buf.swap(i, j);
        permutation(n, i + 1, buf, f);
        buf.swap(i, j);
    }
}

fn main() {
    input! {
        n: usize,
        s: u32,
        t: u32,
        abcd: [(i32, i32, i32, i32); n],
    }
    let mut buf = std::array::from_fn::<_, 6, _>(|i| i as u8);
    let buf = &mut buf[..n];
    let mut ans = f64::INFINITY;
    permutation(n, 0, buf, &mut |p| {
        let mut min_time = f64::INFINITY;
        for dir in 0..1u32 << n {
            let mut pos = (0, 0);
            let mut time = 0.0;
            for &i in p {
                let i = i as usize;
                let dir = ((dir >> i) & 1) != 0;
                let (a, b, c, d) = abcd[i];
                let length = (a as f64 - c as f64).hypot(b as f64 - d as f64);
                if dir {
                    time += (a as f64 - pos.0 as f64).hypot(b as f64 - pos.1 as f64) / s as f64;
                    pos = (c, d);
                } else {
                    time += (c as f64 - pos.0 as f64).hypot(d as f64 - pos.1 as f64) / s as f64;
                    pos = (a, b);
                }
                time += length / t as f64;
            }
            min_time = min_time.min(time);
        }
        ans = ans.min(min_time);
    });
    println!("{}", ans);

}
