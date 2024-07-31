use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); n],
    }

    let mut l_sum = lr.iter().map(|&(l, _)| l).sum::<i64>();
    if l_sum > 0 {
        println!("No");
        return;
    }
    let mut ans = Vec::with_capacity(n);
    let mut lr_iter = lr.iter();
    while let Some(&(l, r)) = lr_iter.next() {
        let next_sum = l_sum + r - l;
        if next_sum >= 0 {
            ans.push(l - l_sum);
            ans.extend(lr_iter.map(|&(l, _)| l));
            println!("Yes");
            for ai in &ans {
                print!("{} ", ai);
            }
            println!();
            // for (&ai, &(li, ri)) in ans.iter().zip(&lr) {
            //     assert!((li..=ri).contains(&ai));
            // }
            // eprintln!("sum: {}", ans.iter().sum::<i64>());
            return;
        }
        l_sum = next_sum;
        ans.push(r);
    }
    println!("No");
}
