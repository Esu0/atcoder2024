use proconio::input;

macro_rules! chmax {
    ($d:expr, $s:expr) => {
        $d = std::cmp::max($d, $s);
    };
}

fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, u64); n],
    }
    dcs.sort_unstable_by_key(|&(d, _, _)| d);
    let mut dpi = vec![i64::MIN; 5001];
    let mut dpi_next = vec![i64::MIN; 5001];
    dpi[0] = 0;
    for &(di, ci, si) in dcs.iter() {
        dpi_next.clone_from_slice(&dpi);
        for (j, &dpij) in dpi.iter().enumerate() {
            let nj = j + ci;
            if nj > di {
                break;
            }
            chmax!(dpi_next[nj], dpij + si as i64);
        }
        dpi.clone_from_slice(&dpi_next);
    }
    let ans = dpi.iter().copied().max().unwrap();
    println!("{}", ans);
}
