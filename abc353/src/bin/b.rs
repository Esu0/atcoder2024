use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [u32; n],
    }
    let mut ans = 0;
    let mut i = 0;
    let mut space = k;
    while i < n {
        let ai = a[i];
        if ai <= space {
            space -= ai;
            i += 1;
        } else {
            ans += 1;
            space = k;
        }
    }
    if space < k {
        ans += 1;
    }
    println!("{ans}");
}
