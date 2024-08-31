use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        tax: [(u8, usize, usize); m],
    }
    let mut rows = vec![true; h];
    let mut r_count = h;
    let mut cols = vec![true; w];
    let mut c_count = w;
    let mut ans = vec![0; 200001];
    for &(t, a, x) in tax.iter().rev() {
        match t {
            1 => {
                if rows[a - 1] {
                    ans[x] += c_count;
                    r_count -= 1;
                    rows[a - 1] = false;
                }
            }
            2 => {
                if cols[a - 1] {
                    ans[x] += r_count;
                    c_count -= 1;
                    cols[a - 1] = false;
                }
            }
            _ => unreachable!(),
        }
    }
    let col0 = h * w - ans.iter().sum::<usize>();
    ans[0] += col0;
    let ans = ans
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, x)| x > 0)
        .collect::<Vec<_>>();
    println!("{}", ans.len());
    for (i, x) in ans {
        println!("{} {}", i, x);
    }
}
