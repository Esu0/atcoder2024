use proconio::input;
use segtree::{operation::Operator, Segtree};

fn main() {
    input! {
        n: usize,
        lu: [(u64, u64); n],
        q: usize,
    }
    #[derive(Clone, Copy, Debug)]
    enum Elem {
        Range(u64, u64, u64),
        LR(u64, u64, u64),
    }

    struct Op;
    impl Operator for Op {
        type Query = Elem;
        const IDENT: Self::Query = Elem::Range(u64::MAX, 0, 0);
        fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
            match (*a, *b) {
                (Elem::Range(ua, la, da), Elem::Range(ub, lb, db)) => {
                    if la > ub {
                        Elem::LR(la, ub, da.saturating_add(db).saturating_add(la - ub))
                    } else if lb > ua {
                        Elem::LR(ua, lb, da + db + (lb - ua))
                    } else {
                        Elem::Range(ua.min(ub), la.max(lb), da + db)
                    }
                }
                (Elem::LR(la, ra, da), Elem::Range(ub, lb, db)) => {
                    if lb > ra {
                        Elem::LR(la, lb, da + db + lb - ra)
                    } else if ub < ra {
                        Elem::LR(la, ub, da + db + ra - ub)
                    } else {
                        Elem::LR(la, ra, da + db)
                    }
                }
                (Elem::Range(ua, la, da), Elem::LR(lb, rb, db)) => {
                    if la > lb {
                        Elem::LR(la, rb, da + db + la - lb)
                    } else if ua < lb {
                        Elem::LR(ua, rb, da + db + lb - ua)
                    } else {
                        Elem::LR(lb, rb, da + db)
                    }
                }
                (Elem::LR(la, ra, da), Elem::LR(lb, rb, db)) => {
                    Elem::LR(la, rb, da + db + ra.abs_diff(lb))
                }
            }
        }
    }

    let seg = Segtree::from_iter_op(lu.iter().map(|&(l, u)| Elem::Range(u, l, 1)), Op);
    for _ in 0..q {
        input! {
            sx: usize,
            sy: u64,
            tx: usize,
            ty: u64,
        }
        let (sx, sy, tx, ty) = if sx > tx {
            (tx - 1, ty, sx, sy)
        } else {
            (sx - 1, sy, tx, ty)
        };
        let elem = seg.query(sx..tx);
        let ans = match elem {
            Elem::Range(u, l, d) => {
                let sy1 = sy.clamp(l, u);
                let ty1 = ty.clamp(l, u);
                let s = sy1.abs_diff(sy);
                let t = ty1.abs_diff(ty);
                s + d + sy1.abs_diff(ty1) - 1 + t
            }
            Elem::LR(l, r, d) => {
                let s = sy.abs_diff(l);
                let t = ty.abs_diff(r);
                s + d - 1 + t
            }
        };
        println!("{}", ans);
    }
}
