use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
        t: marker::Bytes,
    }
    let mut count_s = 0u32;
    let mut count_t = 0u32;
    let mut diff = vec![0; n];
    for ((&si, &ti), diffi) in s.iter().zip(t.iter()).zip(diff.iter_mut()) {
        if si == b'0' && ti == b'1' {
            count_s += 1;
            *diffi = 1;
        }
        if si == b'1' && ti == b'0' {
            count_t += 1;
            *diffi = 1;
        }
    }
    eprintln!("count_s: {}, count_t: {}", count_s, count_t);
    if count_s.abs_diff(count_t) % 2 != 0 {
        println!("-1");
    } else {
        let mut u = vec![0u8; n];
        let flg = count_s > count_t;
        let mut count = count_s.abs_diff(count_t) / 2;
        let s = if flg { s } else { t };
        for (i, &si) in s.iter().enumerate().rev() {
            if si == b'0' && diff[i] == 1 && count > 0 {
                u[i] = 1;
                count -= 1;
            }
        }
        u.iter_mut().for_each(|ui| *ui += b'0');
        println!("{}", std::str::from_utf8(&u).unwrap());
    }
}
