use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut a: [i64; n],
        }
        let mut sum = a[0];
        for i in 1..n - 1 {
            let d = (sum + i as i64 - 1) / i as i64;
            if a[i] >= d {
                sum += a[i];
            } else {
                let diff = d - a[i];
                a[i + 1] -= diff;
                sum += d;
            }
        }
        let d = (sum + n as i64 - 2) / (n - 1) as i64;
        if d > a[n - 1] {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
