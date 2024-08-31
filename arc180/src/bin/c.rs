use proconio::input;
use util::ModInt;

const MODULO: u32 = 1_000_000_007;
type MInt = ModInt<MODULO>;

fn main() {
    input! {
        n: usize,
        a: [i8; n],
    }

    // dp[i][j]: i番目の要素までの数列に対して、総和がA[i] + jとなる部分列で、A[i]を含むものを累積和で置き換えた後としてあり得る数列の個数
    let a_max = a.iter().map(|&x| x.abs()).max().unwrap() as usize;
    let offset1 = a_max * n;
    let mut dp = vec![vec![MInt::new(0); 2 * offset1 + 1]; n];
    let mut set = [false; 21];
    let set = &mut set[..a_max * 2 + 1];
    let offset2 = a_max as i8;
    for i in 1..n {
        if a[i - 1] != 0 {
            set[(a[i - 1] + offset2) as usize] = true;
        }
        for j in set.iter().enumerate().filter(|(_, &b)| b).map(|(j, _)| j) {
            let j = ((j as i8 - offset2) as isize + offset1 as isize) as usize;
            dp[i][j] = MInt::new(1);
        }
    }

    let u = a_max as i32 * n as i32;
    let to_val = |i: usize| i as i32 - offset1 as i32;
    let to_ind = |v: i32| (v + offset1 as i32) as usize;
    for i in 0..n - 1 {
        for j in 0..2 * offset1 + 1 {
            let v = to_val(j) + a[i] as i32;
            if !(-u <= v && v <= u) {
                continue;
            }
            if v != 0 {
                let l = to_ind(v);
                for k in i + 1..n {
                    let a = dp[i][j];
                    dp[k][l] += a;
                }
            } else {
                set.fill(false);
                for k in i + 2..n {
                    if a[k - 1] != 0 {
                        set[(a[k - 1] + offset2) as usize] = true;
                    }
                    for l in set.iter().enumerate().filter(|(_, &b)| b).map(|(l, _)| l) {
                        let l = ((l as i8 - offset2) as isize + offset1 as isize) as usize;
                        let a = dp[i][j];
                        dp[k][l] += a;
                    }
                }
            }
        }
    }
    // dp.iter().for_each(|x| eprintln!("{:?}", x));
    // eprintln!("{:?}", dp);
    let ans = dp
        .iter()
        .flat_map(|x| x[..offset1].iter().chain(&x[offset1 + 1..]))
        .fold(MInt::new(0), |acc, &x| acc + x);
    println!("{}", ans + MInt::new(1));
}
