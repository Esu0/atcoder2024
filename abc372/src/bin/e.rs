use proconio::input;
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut sets = (0..n)
        .map(|i| (1, [i + 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
        .collect::<Vec<_>>();
    let mut uf = UnionFind::new(vec![(); n]);
    // let mut map = (0..n).collect::<Vec<_>>();
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    u: usize,
                    v: usize,
                }
                let su = uf.find_rc(u - 1);
                let sv = uf.find_rc(v - 1);
                if su != sv {
                    uf.unite(u - 1, v - 1);
                    let mut new_set = [0; 10];
                    let u_set = sets[su];
                    let v_set = sets[sv];
                    let mut i = 0;
                    let mut j = 0;
                    let mut k = 0;
                    let count = u_set.0 + v_set.0;
                    for _ in 0..count {
                        if k >= 10 {
                            break;
                        }
                        use std::cmp::Ordering::*;
                        match u_set.1[i].cmp(&v_set.1[j]) {
                            Less => {
                                new_set[k] = v_set.1[j];
                                j += 1;
                            }
                            Equal => {
                                new_set[k] = u_set.1[i];
                                i += 1;
                                j += 1;
                            }
                            Greater => {
                                new_set[k] = u_set.1[i];
                                i += 1;
                            }
                        }
                        k += 1;
                    }
                    sets[uf.find(u - 1)] = (k, new_set);
                    // map[v - 1] = su;
                }
            }
            2 => {
                input! {
                    v: usize,
                    k: usize,
                }

                let sv = uf.find_rc(v - 1);
                // eprintln!("{:?}", sets[sv]);
                if sets[sv].0 < k {
                    println!("-1");
                } else {
                    println!("{}", sets[sv].1[k - 1]);
                }
            }
            _ => unreachable!(),
        }
    }
}
