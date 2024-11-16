use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        st: [(u64, u64); n - 1],
    }
    for (i, &(si, ti)) in st.iter().enumerate() {
        a[i + 1] += ti * (a[i] / si);
    }
    println!("{}", a[n - 1]);
}
