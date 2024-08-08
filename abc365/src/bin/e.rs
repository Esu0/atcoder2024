use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut b = 1;
    let mut ans = 0;
    for i in 0..27 {
        let mut sum = 0;
        let mut xor_sum = (a[0] >> i) & 1;
        for &ai in &a[1..] {
            xor_sum ^= (ai >> i) & 1;
            sum += xor_sum;
        }
        // eprintln!("{xor_sum}");
        let mut part_ans = 0;
        for (j, w) in a[..n - 1].windows(2).enumerate() {
            let &[a1, a2] = w else { unreachable!() };
            part_ans += sum;
            if a1 & b == 0 {
                sum -= (a2 >> i) & 1;
            } else {
                sum = (n - 1 - j) as u64 - sum - ((a2 >> i) & 1);
            }
        }
        part_ans += sum;
        // eprintln!("{part_ans}");
        ans += part_ans * b;
        b <<= 1;
    }
    println!("{ans}");
}
