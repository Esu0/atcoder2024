use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
        }
        if k == 1 {
            for _ in 0..n {
                print!("{} ", 1);
            }
            println!();
            continue;
        }
        let ans = (0..n).map(|i| {
            let mut a = 1u64 << (k - 1);
            a |= (i as u64).reverse_bits() >> (64 - k + 1);
            a
        });
        for a in ans {
            // eprintln!("{:01$b}", a, k);
            print!("{} ", a);
        }
        println!();
    }
}
