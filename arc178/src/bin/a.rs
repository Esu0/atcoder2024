use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }
    if a.contains(&1) || a.contains(&n) {
        println!("-1");
        return;
    }

    let mut p = (1..=n).collect::<Vec<_>>();
    a.sort_unstable();
    for &ai in &a {
        p.swap(ai - 1, ai);
    }
    print!("{}", p[0]);
    for &pi in &p[1..] {
        print!(" {}", pi);
    }
    println!();
}
