use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [u32; n],
        b: [u32; n],
    }
    let diff_sum = a.iter().zip(&b).map(|(&ai, &bi)| ai.abs_diff(bi)).sum::<u32>();
    if k < diff_sum {
        println!("No");
    } else if (k - diff_sum) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
