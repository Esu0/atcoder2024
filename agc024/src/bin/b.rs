use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut pos = vec![0; n];
    for (i, &pi) in p.iter().enumerate() {
        pos[pi - 1] = i;
    }
    let mut ans = 0;
    let mut count = 0;
    let mut prev = 0;
    for &posi in &pos {
        if posi >= prev {
            count += 1;
        } else {
            ans = ans.max(count);
            count = 1;
        }
        prev = posi;
    }
    ans = ans.max(count);

    println!("{}", n - ans);
}
