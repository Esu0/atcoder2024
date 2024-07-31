use std::collections::HashSet;

use proconio::input;

#[allow(dead_code)]
fn answer_test(n: usize, m: usize, ans: &[(usize, usize)]) -> bool {
    if !ans.iter().all(|&(a, b)| (1..=n).contains(&a) && (1..=n).contains(&b)) {
        return false;
    }
    let mut count1 = vec![0usize; n];
    let mut count2 = vec![0usize; n];
    for &(a, b) in ans {
        count1[a - 1] += 1;
        count2[b - 1] += 1;
    }
    count1.iter().all(|&c| c == m) && count2.iter().all(|&c| c == m)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut sum_set = ab.iter().map(|&(a, b)| (a - 1 + b - 1) % n).collect::<HashSet<_>>();
    let mut i = 0;
    while sum_set.len() < m {
        while !sum_set.insert(i) {
            i += 1;
        }
    }
    // eprintln!("{:?}", sum_set);
    let mut ans = vec![];

    for &sum in &sum_set {
        for i in 0..n {
            let j = (sum + n - i) % n;
            ans.push((i + 1, j + 1));
        }
    }
    // assert!(answer_test(n, m, &ans));
    println!("{}", ans.len());

    for &(ai, bi) in &ans {
        println!("{} {}", ai, bi);
    }
}
