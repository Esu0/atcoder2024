use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32; m],
        x: [[u32; m]; n]
    }
    for row in &x {
        for (ai, &xi) in a.iter_mut().zip(row) {
            *ai = ai.saturating_sub(xi);
        }
    }
    if a.iter().all(|&ai| ai == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
