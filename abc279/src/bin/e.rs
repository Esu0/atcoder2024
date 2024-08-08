use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }
    a.iter_mut().for_each(|ai| *ai -= 1);
    let mut b = (0..n).collect::<Vec<_>>();
    let mut pos = 0;
    for &ai in a.iter().rev() {
        b.swap(ai, ai + 1);
        // eprintln!("{:?}", b);
    }
    for &ai in &a {
        b.swap(ai, ai + 1);
        // eprintln!("{:?}", pos);
        println!("{}", b[pos] + 1);
        if pos == ai {
            pos += 1;
        } else if pos == ai + 1 {
            pos -= 1;
        }
    }
}
