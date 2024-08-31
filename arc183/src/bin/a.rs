use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    if n == 1 {
        for _ in 0..k {
            print!("1 ");
        }
    } else if n % 2 == 0 {
        let a = n / 2;
        print!("{} ", a);
        for i in (1..=n).rev() {
            let k = if i == a { k - 1 } else { k };
            for _ in 0..k {
                print!("{} ", i);
            }
        }
    } else {
        let a = n / 2 + 1;
        for _ in 0..k {
            print!("{} ", a);
        }
        print!("{} ", a - 1);
        for i in (1..=n).rev() {
            if i == a {
                continue;
            }
            let k = if i == a - 1 { k - 1 } else { k };
            for _ in 0..k {
                print!("{} ", i);
            }
        }
    }
    println!();
}
