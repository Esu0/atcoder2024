use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        a: [u8; n],
        s: marker::Bytes,
    }
    let mut m_count = Vec::with_capacity(n);
    let mut count = [0u32, 0, 0];
    for i in 0..n {
        if s[i] == b'M' {
            count[a[i] as usize] += 1;
        }
        m_count.push(count);
    }

    let mut x_count = vec![[0, 0, 0]; n];
    let mut count = [0u32, 0, 0];
    for i in (0..n).rev() {
        if s[i] == b'X' {
            count[a[i] as usize] += 1;
        }
        x_count[i] = count;
    }
    // eprintln!("{:?}", m_count);
    // eprintln!("{:?}", x_count);

    let mut sum = 0u64;
    fn mex(a: u8, b: u8, c: u8) -> u8 {
        if ![a, b, c].contains(&0) {
            0
        } else if ![a, b, c].contains(&1) {
            1
        } else if ![a, b, c].contains(&2) {
            2
        } else {
            3
        }
    }

    for j in 1..n - 1 {
        if s[j] == b'E' {
            let iteration = [[0, 0], [0, 1], [0, 2], [1, 0], [1, 1], [1, 2], [2, 0], [2, 1], [2, 2]];
            for [i, k] in iteration {
                sum += mex(i, a[j], k) as u64 * m_count[j - 1][i as usize] as u64 * x_count[j + 1][k as usize] as u64;
            }
        }
    }

    println!("{sum}");
}
