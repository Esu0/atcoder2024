use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut rc: [(usize, usize); n],
    }
    rc.sort_unstable();
    let mut l = Vec::<Vec<(usize, usize)>>::with_capacity(n);
    for &(_, c) in &rc {
        let k = l.partition_point(|x| x.last().unwrap().0 <= c);
        if k == l.len() {
            let i = if k == 0 {
                0
            } else {
                l[k - 1].len() - 1
            };
            l.push(vec![(c, i)]);
        } else {
            let i = if k == 0 {
                0
            } else {
                l[k - 1].len() - 1
            };
            l[k].push((c, i));
        }
    }
    let (c, i) = *l.last().unwrap().last().unwrap();
    eprintln!("{}", c);
    let mut prev_i = i;
    for v in l.iter().rev().skip(1) {
        let (c, next_i) = v[prev_i];
        eprintln!("{}", c);
        prev_i = next_i;
    }
}
