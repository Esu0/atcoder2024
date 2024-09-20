use proconio::input;

fn answer_array<I>(ans: I)
where 
    I: IntoIterator,
    I::Item: std::fmt::Display,
{
    let mut ans = ans.into_iter();
    if let Some(first) = ans.next() {
        print!("{}", first);
        for x in ans {
            print!(" {}", x);
        }
    }
    println!();
}

fn main() {
    input! {
        n: usize,
        m: usize,
        x1: u64,
    }
    // 各頂点における最も遅い到着時刻
    let mut t = vec![0; n];
    let mut x = vec![0; m];
    x[0] = x1;
    let mut events = Vec::with_capacity(2 * m);
    for i in 0..m {
        input! {
            a: usize,
            b: usize,
            s: u64,
            t: u64,
        }
        // 出発
        events.push((s, a - 1, 1u8, i));
        // 到着
        events.push((t, b - 1, 0u8, i));
    }
    events.sort_unstable_by_key(|&(time, _, k, _)| (time, k));
    // eprintln!("{:?}", events);
    for &(time, v, k, e) in &events {
        if k == 0 {
            // 到着イベント
            t[v] = t[v].max(time + x[e]);
        } else {
            // 出発イベント
            if e != 0 {
                x[e] = t[v].saturating_sub(time);
            }
        }
    }
    answer_array(x.iter().skip(1));
}
