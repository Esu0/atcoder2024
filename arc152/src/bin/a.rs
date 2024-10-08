use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }

    let mut count = l;
    for &ai in &a {
        if ai == 2 && count < ai {
            println!("No");
            return;
        }
        count = count.saturating_sub(ai + 1);
    }
    println!("Yes");
}
