use proconio::input;

fn main() {
    input! {
        n: usize,
        mut hwd: [(usize, usize, usize); n],
    }

    let mut hwd = hwd.iter().map(|&(h, w, d)| {
        [h, w, d]
    }).collect::<Vec<_>>();
    for i in 0..n {
        hwd[i].sort_unstable();
    }
    
}
