use proconio::input;

fn permutation<T>(n: usize, i: usize, buf: &mut [T], f: &mut impl FnMut(&[T])) {
    if i == n {
        f(buf);
        return;
    }
    for j in i..n {
        buf.swap(i, j);
        permutation(n, i + 1, buf, f);
        buf.swap(i, j);
    }
}

fn main() {
    let mut c = [[0; 3]; 3];
    for cij in c.iter_mut().flat_map(|ci| ci.iter_mut()) {
        input! {
            tmp: u8,
        }
        *cij = tmp;
    }
    
    let mut count = 0;
    let mut buf = std::array::from_fn::<_, 9, _>(|i| i + 1);
    permutation(9, 0, &mut buf, &mut |p| {
        
    });
}
