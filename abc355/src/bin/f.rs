use proconio::input;
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        q: usize,
        abc: [(usize, usize, usize); n - 1],
    }
    let mut ufs = std::array::from_fn::<_, 10, _>(|_| UnionFind::new(vec![(); n]));
    let mut ans = 0;
    for (a, b, c) in abc {
        for uf in &mut ufs[c - 1..] {
            uf.unite(a - 1, b - 1);
        }
        ans += c;
    }

    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
            w: usize,
        }
        let u = u - 1;
        let v = v - 1;
        let mut x = 0;
        for (i, uf) in ufs.iter_mut().enumerate() {
            if uf.find_rc(u) == uf.find_rc(v) {
                x = i + 1;
                break;
            }
        }
        if x > w {
            ans = ans - x + w;
            for uf in &mut ufs[w - 1..] {
                uf.unite(u, v);
            }
        }

        println!("{}", ans);
    }
}
