use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, char); m],
    }
    let mut c = vec![false; n];
    for &(a, b) in &ab {
        if b == 'M' && !c[a - 1] {
            c[a - 1] = true;
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
