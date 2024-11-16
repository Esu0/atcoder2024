use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;
use segtree::{operation, Segtree};
use util::ModInt;
type MInt = ModInt<998244353>;
struct Operation;
struct Add;
impl Monoid for Add {
    type S = (MInt, usize);
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }

    fn identity() -> Self::S {
        (MInt::new(0), 1)
    }
}

impl MapMonoid for Operation {
    type F = MInt;
    type M = Add;
    fn identity_map() -> Self::F {
        MInt::new(0)
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + *g
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        (x.0 + MInt::new(x.1 as _) * *f, x.1)
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }
    let mut segtree = (0..n)
        .map(|_| 0usize)
        .collect::<Segtree<_, operation::Add<_>>>();
    let pos = {
        let mut res = vec![usize::MAX; n];
        for (i, &pi) in p.iter().enumerate() {
            res[pi - 1] = i;
        }
        res
    };
    let mut sum = 0;
    for &posi in &pos {
        sum += segtree.query(posi + 1..);
        segtree.update(posi, 1);
    }

    let mut segtree = LazySegtree::<Operation>::from(vec![(MInt::new(0), 0); n]);
    for &pi in &p[..k] {
        let pi = pi - 1;
        segtree.apply_range(pi + 1.., MInt::new(1));
        segtree.set(pi, (MInt::new(0), 1));
    }
    let inv = sum;
    let mut sum = segtree.all_prod().0;
    // eprintln!("{sum}");
    // for i in 0..n {
    //     let elem = segtree.get(i);
    //     eprint!("{} {}, ", elem.0, elem.1);
    // }
    // eprintln!();

    for (&pi, &pj) in p.iter().zip(&p[k..]) {
        let pi = pi - 1;
        let pj = pj - 1;
        segtree.set(pi, (MInt::new(0), 0));
        segtree.apply_range(pj + 1.., MInt::new(1));
        segtree.set(pj, (MInt::new(0), 1));
        sum += segtree.all_prod().0;
        // eprintln!("{sum}");
    }
    let ans = MInt::new(inv as _) - sum / MInt::new((n - k + 1) as _)
        + MInt::new((k * (k - 1)) as _) / MInt::new(4);
    println!("{}", ans);
}
