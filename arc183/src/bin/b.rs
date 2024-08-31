use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    'next_case: for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            a: [usize; n],
            b: [usize; n],
        }
        if a == b {
            println!("Yes");
            continue;
        }
        if k == 1 {
            let mut b2 = Vec::with_capacity(n);
            let mut prev = usize::MAX;
            for &bi in &b {
                if prev != bi {
                    b2.push(bi);
                    prev = bi;
                }
            }
            // eprintln!("{:?}", b2);
            let mut j = 0;
            for &ai in &a {
                if ai == b2[j] {
                    j += 1;
                }
                if j >= b2.len() {
                    println!("Yes");
                    continue 'next_case;
                }
            }
            println!("No");
        } else {
            let mut s = vec![false; n];
            for &ai in &a {
                s[ai - 1] = true;
            }

            if !b.iter().all(|&bi| s[bi - 1]) {
                println!("No");
                continue;
            }
            let mut m = usize::MAX;
            let mut pos = vec![usize::MAX; n];
            for (i, &bi) in b.iter().enumerate() {
                if pos[bi - 1] != usize::MAX {
                    m = m.min(i - pos[bi - 1]);
                }
                pos[bi - 1] = i;
            }

            if m <= k {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
