use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    'next_case: for _ in 0..t {
        input! {
            n: usize,
            mut p: [usize; n],
        }
        p.iter_mut().for_each(|x| *x -= 1);
        if p.iter().enumerate().all(|(i, &pi)| i == pi) {
            println!("0");
            continue;
        }
        let mut min_rev = vec![0; n];
        {
            let mut min = usize::MAX;
            for (i, &pi) in p.iter().enumerate().rev() {
                min = min.min(pi);
                min_rev[i] = min;
            }
        }
        // eprintln!("{:?}", min_rev);
        let mut max = 0;
        for (i, &pi) in p.iter().enumerate() {
            // eprintln!("{max}");
            if i == pi {
                let prev_i = i.wrapping_sub(1);
                if (prev_i >= n || max == prev_i) && (i + 1 >= n || min_rev[i + 1] == i + 1) {
                    println!("1");
                    continue 'next_case;
                }
            }
            max = max.max(i);
        }
        if p.first().copied() == Some(n - 1) && p.last().copied() == Some(0) {
            println!("3");
        } else {
            println!("2");
        }
    }
}
