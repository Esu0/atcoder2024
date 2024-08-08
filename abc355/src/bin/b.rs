use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    }
    let mut c = a.iter().map(|&ai| (ai, true)).chain(b.iter().map(|&bi| (bi, false))).collect::<Vec<_>>();
    c.sort_unstable_by_key(|&(ci, _)| ci);
    for w in c.windows(2) {
        let f1 = w[0].1;
        let f2 = w[1].1;
        if f1 && f2 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
