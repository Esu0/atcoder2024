use proconio::input;
use union_find::UnionFind;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(vec![(); h * w]);
    let mut red = vec![false; h * w];
    let to_ind = |i: usize, j: usize| i * w + j;
    let to_ij = |ind: usize| (ind / w, ind % w);
    let d = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    r: usize,
                    c: usize,
                }
                let r = r - 1;
                let c = c - 1;
                red[to_ind(r, c)] = true;
                for &(di, dj) in &d {
                    let ni = r.wrapping_add(di);
                    let nj = c.wrapping_add(dj);
                    if ni < h && nj < w && red[to_ind(ni, nj)]{
                        uf.unite(to_ind(r, c), to_ind(ni, nj));
                    }
                }
            }
            2 => {
                input! {
                    ra: usize,
                    ca: usize,
                    rb: usize,
                    cb: usize,
                }
                let ind_a = to_ind(ra - 1, ca - 1);
                let ind_b = to_ind(rb - 1, cb - 1);
                if red[ind_a] && red[ind_b] && uf.find_rc(ind_a) == uf.find_rc(ind_b) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => unreachable!(),
        }
    }
}
