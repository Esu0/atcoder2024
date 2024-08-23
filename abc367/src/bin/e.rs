use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        x: [usize; n],
        mut a: [u32; n],
    }

    let mut k = k;
    let mut x = x;
    let mut buf = vec![0; n];
    let mut buf2 = vec![0; n];
    x.iter_mut().for_each(|xi| *xi -= 1);
    while k > 0 {
        if k & 1 != 0 {
            buf2.clone_from_slice(&a);
            for (ai, &xi) in a.iter_mut().zip(&x) {
                *ai = buf2[xi];
            }
        }
        buf.clone_from_slice(&x);
        for xi in x.iter_mut() {
            *xi = buf[*xi];
        }
        k >>= 1;
    }
    for &ai in &a {
        print!("{ai} ");
    }
    println!();
}
