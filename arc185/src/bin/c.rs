use proconio::input;
use ac_library::convolution;
use ac_library::modint::ModInt998244353;
type MInt = ModInt998244353;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let mut b = vec![MInt::raw(0); x + 1];
    for &ai in &a {
        if b[ai].val() != 3 {
            b[ai] += 1;
        }
    }
    let c = convolution(&b, &b);
    let c = c.iter().enumerate().map(|(i, &ci)| (ci.val() - if i % 2 == 0 { b[i / 2].val() } else { 0 }) as usize/ 2).collect::<Vec<_>>();
    let mut ai = usize::MAX;
    for p in 1..x {
        if b[p].val() > 0 {
            b[p] -= 1;
            let k = x - p;
            let mut ck = c[k];
            if k >= p {
                let t = k - p;
                ck -= b[t].val() as usize;
            }
            if ck > 0 {
                ai = p;
                break;
            }
            b[p] += 1;
        }
    }
    if ai == usize::MAX {
        println!("-1");
        return;
    }

    let t = x - ai;
    let mut aj = usize::MAX;
    for q in 1..t {
        if b[q].val() > 0 {
            b[q] -= 1;
            if b[t - q].val() > 0 {
                aj = q;
                break;
            }
        }
    }
    assert!(aj != usize::MAX);

    let ak = t - aj;
    let (mut i, mut j, mut k) = (usize::MAX, usize::MAX, usize::MAX);
    for (l, &al) in a.iter().enumerate() {
        if i == usize::MAX && al == ai {
            i = l + 1;
        } else if j == usize::MAX && al == aj {
            j = l + 1;
        } else if k == usize::MAX && al == ak {
            k = l + 1;
        }
    }
    assert!(i != usize::MAX && j != usize::MAX && k != usize::MAX);
    let mut idx = [i, j, k];
    idx.sort_unstable();
    assert!(a[i - 1] + a[j - 1] + a[k - 1] == x);
    println!("{} {} {}", idx[0], idx[1], idx[2]);
}
