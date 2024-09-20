use proconio::input;
use segtree::{operation, Segtree};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
        q: usize,
    }

    let mut seg_a = a.iter().copied().collect::<Segtree<_, operation::Add<_>>>();
    let mut seg_b = b.iter().copied().collect::<Segtree<_, operation::Max<_>>>();
    for _ in 0..q {
        input! {
            t: u8,
        }

        match t {
            1 => {
                input! {
                    i: usize,
                    x: u64,
                }
                *seg_a.get_mut(i - 1) = x;
            }
            2 => {
                input! {
                    i: usize,
                    x: u64,
                }
                *seg_b.get_mut(i - 1) = x;
            }
            3 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let mut ans = 0;
                let mut i = l - 1;
                loop {
                    let next_i = seg_b.upper_bound(i, |&x| {
                        x <= 1
                    });
                    // dbg!(next_i);
                    if next_i < r {
                        ans += seg_a.query(i..next_i);
                        ans = (ans + seg_a[next_i]).max(ans * seg_b[next_i]);
                        i = next_i + 1;
                    } else {
                        ans += seg_a.query(i..r);
                        break;
                    }
                }
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
