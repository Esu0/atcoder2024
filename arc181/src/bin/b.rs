use proconio::{input, marker};

#[allow(dead_code)]
mod z_algorithm {
    use std::ops::Deref;

    pub struct ZArray {
        data: Vec<usize>,
    }

    impl Deref for ZArray {
        type Target = [usize];

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl ZArray {
        pub fn new<T: Eq>(slice: &[T]) -> Self {
            let mut z = Vec::with_capacity(slice.len() - 1);
            let (mut l, mut r) = (0, 1);
            while z.len() < slice.len() - 1 {
                let i = z.len() + 1;
                if z.get(i - l - 1).is_some_and(|&x| x + i < r) {
                    z.push(z[i - l - 1]);
                } else {
                    l = i;
                    r = i.max(r);
                    while slice.get(r).is_some_and(|x| x == &slice[r - l]) {
                        r += 1;
                    }
                    z.push(r - l);
                }
            }
            Self { data: z }
        }

        pub fn into_vec(self) -> Vec<usize> {
            self.data
        }
    }
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            s: marker::Bytes,
            x: marker::Bytes,
            y: marker::Bytes,
        }
        let z = z_algorithm::ZArray::new(&s);
        let mut pattern_num = 1;
        for i in 1..s.len() {
            if s.len() % i == 0 && z[i - 1] + i == s.len() {
                pattern_num = s.len() / i;
                break;
            }
        }

        // eprintln!("{}", pattern_num);
        let mut x_count = [0u32, 0];
        let mut y_count = [0u32, 0];
        for &xi in &x {
            match xi {
                b'0' => x_count[0] += 1,
                b'1' => x_count[1] += 1,
                _ => unreachable!(),
            }
        }
        for &yi in &y {
            match yi {
                b'0' => y_count[0] += 1,
                b'1' => y_count[1] += 1,
                _ => unreachable!(),
            }
        }
        // eprintln!("{:?}", x_count);
        // eprintln!("{:?}", y_count);
        let p = pattern_num as i64 * (x_count[0] as i64 - y_count[0] as i64);
        let q = y_count[1] as i64 - x_count[1] as i64;
        // dbg!(p, q);
        if q == 0 {
            if p == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        } else if p.abs() % q.abs() == 0 {
            let ans = p / q;
            if ans >= 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    }
}
