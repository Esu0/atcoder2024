use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        p: usize,
        mut l: [u32; n],
    }

    let mut d = 0;
    while l.iter().filter(|&&li| li >= t).count() < p {
        d += 1;
        l.iter_mut().for_each(|li| *li += 1);
    }
    println!("{}", d);
}
