use proconio::input;
use rand::Rng;
use util::IteratorExt;
const MODULO: u64 = (1 << 61) - 1;
/// a * b % (2^61 - 1)
fn mul_rem(a: u64, b: u64) -> u64 {
    const MASK31: u64 = (1 << 31) - 1;
    const MASK30: u64 = (1 << 30) - 1;
    let au = a >> 31;
    let bu = b >> 31;
    let ad = a & MASK31;
    let bd = b & MASK31;
    let mid = ad * bu + au * bd;
    (au * bu * 2 + (mid >> 30) + ((mid & MASK30) << 31) + ad * bd) % MODULO
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut rng = rand::thread_rng();
    let h = (0..n).map(|_| rng.gen_range(1..((1u64 << 61) - 1))).collect::<Vec<_>>();
    let hash_a = a.iter().cumulative_sum(0, |&acc, &ai| (acc + h[ai - 1]) % MODULO).collect::<Vec<_>>();
    let hash_b = b.iter().cumulative_sum(0, |&acc, &bi| (acc + h[bi - 1]) % MODULO).collect::<Vec<_>>();
    // eprintln!("{:?}", hash_a);
    for _ in 0..q {
        input! { l1: usize, r1: usize, l2: usize, r2: usize }
        if r1 - l1 != r2 - l2 {
            println!("No");
        } else {
            let h1 = (hash_a[r1] + MODULO - hash_a[l1 - 1]) % MODULO;
            let h2 = (hash_b[r2] + MODULO - hash_b[l2 - 1]) % MODULO;
            if h1 == h2 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
