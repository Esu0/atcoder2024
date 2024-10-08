use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut p: [usize; n],
        }
        let mut count = 0;
        loop {
            let i = p.iter().enumerate().min_by_key(|&(_, &x)| x).unwrap().0;
            p[..=i].fill(usize::MAX);
            count += 1;
            if p[n - 1] == usize::MAX {
                break;
            }
        }
        println!("{}", count);
    }
}
