use proconio::input;
use std::cmp::Ordering::*;
fn main() {
    input! {
        sab: char,
        sac: char,
        sbc: char,
    }
    let mut arr = [0, 1, 2];
    let mut cmp = [[Equal; 3]; 3];
    if sab == '>' {
        cmp[0][1] = Greater;
        cmp[1][0] = Less;
    } else {
        cmp[0][1] = Less;
        cmp[1][0] = Greater;
    }
    if sac == '>' {
        cmp[0][2] = Greater;
        cmp[2][0] = Less;
    } else {
        cmp[0][2] = Less;
        cmp[2][0] = Greater;
    }
    if sbc == '>' {
        cmp[1][2] = Greater;
        cmp[2][1] = Less;
    } else {
        cmp[1][2] = Less;
        cmp[2][1] = Greater;
    }
    arr.sort_unstable_by(|&x1, &x2| cmp[x1][x2]);
    println!("{}", (b'A' + arr[1] as u8) as char);
}
