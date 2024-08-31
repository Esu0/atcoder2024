use proconio::input;
use segtree::{operation, Segtree};

#[derive(Clone, Copy, Debug, Default)]
struct Xor;
impl operation::Operator for Xor {
    type Query = u32;
    const IDENT: Self::Query = 0;
    fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
        *a ^ *b
    }

    fn op_assign_left(&self, a: &mut Self::Query, b: &Self::Query) {
        *a ^= *b;
    }

    fn op_assign_right(&self, a: &Self::Query, b: &mut Self::Query) {
        *b ^= *a;
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
    }
    let mut segtree = a.iter().copied().collect::<Segtree<_, Xor>>();
    for _ in 0..q {
        input! {
            t: u8,
            x: usize,
        }

        match t {
            1 => {
                input! { y: u32 }
                *segtree.get_mut(x - 1) ^= y;
            }
            2 => {
                input! { y: usize }
                let ans = segtree.query(x - 1..y);
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
