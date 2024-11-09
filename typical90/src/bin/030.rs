use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
    }

    let mut count = vec![0; n + 1];
    for i in 2..=n {
        if count[i] == 0 {
            let mut j = i;
            while j <= n {
                count[j] += 1;
                j += i;
            }
        }
    }
    let ans = count[2..].iter().filter(|&&x| x >= k).count();
    println!("{:?}", ans);
}
