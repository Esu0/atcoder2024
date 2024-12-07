use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [marker::Bytes; h]
    }

    let mut floors = s
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell == b'.')
                .map(move |(j, _)| (i, j))
        })
        .collect::<Vec<_>>();
    
    // let mut buf = vec![];
    let mut ans = 0;
    for (k, &(i1, j1)) in floors.iter().enumerate() {
        for &(i2, j2) in floors[k + 1..].iter() {
            let count = (0..h).flat_map(|i| (0..w).map(move |j| (i, j))).filter(|&(i, j)| {
                i.abs_diff(i1) + j.abs_diff(j1) <= d || i.abs_diff(i2) + j.abs_diff(j2) <= d
            }).filter(|&(i, j)| s[i][j] == b'.').count();
            ans = ans.max(count);
        }
    }
    println!("{ans}");
}
