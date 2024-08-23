use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    }

    let mut data = ab.iter().enumerate().map(|(i, &(a, b))| (a, b, i + 1)).collect::<Vec<_>>();
    data.sort_by(|&(a1, b1, _), &(a2, b2, _)| (a2 * (a1 + b1)).cmp(&(a1 * (a2 + b2))));
    // eprintln!("{:?}", data);
    for &(_, _, i) in &data {
        print!("{i} ");
    }
    println!();
}
