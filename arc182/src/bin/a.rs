use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        pv: [(usize, u32); q],
    }
    use std::cmp::Ordering::*;
    // 0ビット目: 左への操作を禁止
    // 1ビット目: 右への操作を禁止
    let mut labels = vec![0; q];
    for (i, &(pi, vi)) in pv.iter().enumerate() {
        for (j, &(pj, vj)) in pv.iter().enumerate().skip(i + 1) {
            if vj < vi {
                match pi.cmp(&pj) {
                    Less => {
                        labels[i] |= 1;
                        labels[j] |= 2;
                    }
                    Equal => {
                        println!("0");
                        return;
                    }
                    Greater => {
                        labels[i] |= 2;
                        labels[j] |= 1;
                    }
                }
            }
        }
    }

    let mut ans = 1;
    for &l in &labels {
        match l {
            0 => ans = ans * 2 % 998244353,
            1 | 2 => (),
            3 => {
                println!("0");
                return;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
