use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [(usize, char); n],
    }

    let mut m = usize::MAX;
    for l in 1..=100 {
        let mut l = l;
        let mut h = 0;
        for &(a, s) in &a_s {
            if s == 'R' {
                continue;
            }
            h += a.abs_diff(l);
            l = a;
        }
        m = m.min(h);
    }

    let mut m2 = usize::MAX;
    for r in 1..=100 {
        let mut r = r;
        let mut h = 0;
        for &(a, s) in &a_s {
            if s == 'L' {
                continue;
            }
            h += a.abs_diff(r);
            r = a;
        }
        m2 = m2.min(h);
    }
    println!("{}", m + m2);
}
