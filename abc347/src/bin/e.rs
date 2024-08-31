use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }
    let mut set = vec![false; n];
    let mut size = 0usize;
    let mut a = Vec::with_capacity(q);
    for &xi in &x {
        if set[xi - 1] {
            set[xi - 1] = false;
            size -= 1;
        } else {
            set[xi - 1] = true;
            size += 1;
        }
        a.push(size);
    }
    for i in (1..q).rev() {
        a[i - 1] += a[i];
    }
    let mut ans = vec![0; n];
    set.fill(false);
    for (i, &xi) in x.iter().enumerate() {
        if set[xi - 1] {
            set[xi - 1] = false;
            ans[xi - 1] -= a[i];
        } else {
            set[xi - 1] = true;
            ans[xi - 1] += a[i];
        }
    }
    for &ai in &ans {
        print!("{} ", ai);
    }
    println!();
}
