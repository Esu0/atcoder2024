use proconio::input;

fn main() {
    input! {
        n: usize,
        c: u32,
        ta: [(u8, u32); n],
    }
    
    let mut ans = vec![0; n];
    for i in 0..30 {
        let mut x = (c >> i) & 1;
        let mut op = [0, 1];
        for (j, &(t, a)) in ta.iter().enumerate() {
            if t == 1 {
                op[0] &= (a >> i) & 1;
                op[1] &= (a >> i) & 1;
            }
            if t == 2 {
                op[0] |= (a >> i) & 1;
                op[1] |= (a >> i) & 1;
            }
            if t == 3 {
                op[0] ^= (a >> i) & 1;
                op[1] ^= (a >> i) & 1;
            }
            x = op[x as usize];
            ans[j] |= x << i;
        }
    }

    for &a in &ans {
        println!("{}", a);
    }
}
