use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
        c: u64,
    }
    let ai = (0..10000).map(|i| a * i).collect::<Vec<_>>();
    let bj = (0..10000).map(|i| b * i).collect::<Vec<_>>();
    let mut ans = usize::MAX;
    for (i, &ai) in ai.iter().enumerate() {
        for (j, &bj) in bj[..=9999 - i].iter().enumerate() {
            let tmp = ai + bj;
            if n < ai + bj {
                break;
            }
            let rem = n - tmp;
            if rem % c == 0 {
                ans = ans.min(i + j + rem as usize / c as usize);
            }
        }
    }
    println!("{}", ans);
}
