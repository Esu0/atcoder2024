use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
    }

    let mut pos = vec![usize::MAX; n];
    for (i, &pi) in p.iter().enumerate() {
        pos[pi - 1] = i;
    }

    for _ in 0..q {
        input! { a: usize, b: usize }
        println!("{}", std::cmp::min_by_key(a, b, |&x| pos[x - 1]));
    }
}
