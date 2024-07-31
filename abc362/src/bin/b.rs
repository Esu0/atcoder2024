use proconio::input;

fn dprod(v1: (i32, i32), v2: (i32, i32)) -> i32 {
    v1.0 * v2.0 + v1.1 * v2.1
}

fn main() {
    input! {
        xa: i32,
        ya: i32,
        xb: i32,
        yb: i32,
        xc: i32,
        yc: i32,
    }

    let va = (xb - xc, yb - yc);
    let vb = (xc - xa, yc - ya);
    let vc = (xa - xb, ya - yb);
    if dprod(va, vb) == 0 || dprod(vb, vc) == 0 || dprod(vc, va) == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
