use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(usize, u64); t],
    }
    let mut map = HashMap::from([(0, n)]);
    let mut score = vec![0; n];
    for &(ai, bi) in &ab {
        let si = score[ai - 1];
        score[ai - 1] = si + bi;
        let m = map.get_mut(&si).unwrap();
        if *m == 1 {
            map.remove(&si);
        } else {
            *m -= 1;
        }
        map.entry(si + bi).and_modify(|x| *x += 1).or_insert(1);
        println!("{}", map.len());
    }
}
