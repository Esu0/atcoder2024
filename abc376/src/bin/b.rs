use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let (mut l, mut r) = (0, 1);
    let mut ans = 0;
    for _ in 0..q {
        input! {
            h: char,
            t: usize,
        }
        let t = t - 1;
        match h {
            'L' => {
                let t2 = if t < l { t + n } else { t };
                let r2 = if r < l { r + n } else { r };
                assert_ne!(t2, r2);
                if (l..t2).contains(&r2) {
                    ans += (l + n - t2) % n;
                } else {
                    ans += t2 - l;
                }
                l = t;
            }
            'R' => {
                let t2 = if t < r { t + n } else { t };
                let l2 = if l < r { l + n } else { l };
                assert_ne!(t2, l2);
                if (r..t2).contains(&l2) {
                    ans += (r + n - t2) % n;
                } else {
                    ans += t2 - r;
                }
                r = t;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
