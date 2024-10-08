use proconio::{input, marker};

fn main() {
    input! {
        t: usize,
    }

    'next_case: for _ in 0..t {
        input! {
            n: usize,
            s: marker::Bytes,
        }

        for i in 1..n {
            let (s1, s2) = s.split_at(i);
            if s2 > s1 {
                println!("Yes");
                continue 'next_case;
            }
        }
        println!("No");
    }
}
