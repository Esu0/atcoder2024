use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }
    let mut cards: Vec<Vec<usize>> = vec![];
    let mut top = BTreeMap::<usize, usize>::new();
    let mut ans = vec![usize::MAX; n];
    for (i, &pi) in p.iter().enumerate() {
        if let Some((&t, &j)) = top.range(pi..).next() {
            cards[j].push(pi);
            top.remove(&t);
            top.insert(pi, j);
        } else {
            top.insert(pi, cards.len());
            cards.push(vec![pi]);
        }
        let j = *top.get(&pi).unwrap();
        if cards[j].len() == k {
            cards[j].iter().for_each(|&c| {
                ans[c - 1] = i + 1;
            });
            cards[j].clear();
            top.remove(&pi);
        }
    }
    for &a in &ans {
        println!("{}", a as isize);
    }
}
