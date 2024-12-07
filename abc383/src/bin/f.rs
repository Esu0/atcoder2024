use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        k: i64,
        mut puc: [(usize, i64, usize); n],
    }

    puc.sort_unstable_by_key(|&(_, _, c)| c);
    let mut dpi = vec![i64::MIN; x + 1];
    let mut dpi_next = vec![0; x + 1];
    dpi[0] = 0;
    let mut dpi_tmp = dpi.clone();
    let mut current_c = usize::MAX;
    for (pi, ui, ci) in puc {
        if ci != current_c {
            current_c = ci;
            dpi_tmp.clone_from(&dpi);
        }
        dpi_next.clone_from(&dpi);
        for j in 0..=x - pi {
            dpi_next[j + pi] = dpi_next[j + pi].max(dpi[j] + ui).max(dpi_tmp[j] + ui + k);
        }
        dpi.clone_from(&dpi_next);
    }

    let ans = dpi.iter().copied().max().unwrap();
    println!("{ans}");

}
