use proconio::input_interactive;

// 硬貨xと硬貨yが同種ならfalse, 異種ならtrue
fn query(x: usize, y: usize) -> bool {
    println!("? {} {}", x + 1, y + 1);
    input_interactive! {
        b: i8,
    }
    match b {
        1 => true,
        0 => false,
        -1 => panic!(),
        _ => unreachable!(),
    }
}

fn main() {
    input_interactive! {
        _: usize,
        _: usize,
        _: usize,
    }
    let n = 1000usize;
    let m = 10usize;
    let q = 950usize;
    // let mut a = (0..n).map(|i| vec![i]).collect::<Vec<_>>();
    let mut b = vec![];
    let mut c = vec![];
    for i in 0..n / 2 {
        if query(i * 2, i * 2 + 1) {
            c.push([vec![i * 2], vec![i * 2 + 1]]);
        } else {
            b.push([i * 2, i * 2 + 1]);
        }
    }
    // c.len() <= 10
    let mut d = vec![];
    while b.len() > 1 {
        let c1 = b.pop().unwrap();
        let c2 = b.pop().unwrap();
        if query(c1[0], c2[0]) {
            c.push([c1.to_vec(), c2.to_vec()]);
        } else {
            d.push(c1.iter().chain(&c2).copied().collect::<Vec<_>>());
        }
    }
    if b.len() == 1 {
        let mut tmp = d.pop().unwrap();
        if query(b[0][0], tmp[0]) {
            c.push([b.pop().unwrap().to_vec(), tmp]);
        } else {
            tmp.extend(b.pop().unwrap());
            d.push(tmp);
        }
    }
    // d: 長さ4以上の配列の配列
    let mut e = vec![];
    while d.len() > 1 {
        let mut c1 = d.pop().unwrap();
        let c2 = d.pop().unwrap();
        if query(c1[0], c2[0]) {
            c.push([c1, c2]);
        } else {
            c1.extend(c2);
            e.push(c1);
        }
    }
    if d.len() == 1 {
        let mut tmp = e.pop().unwrap();
        if query(d[0][0], tmp[0]) {
            c.push([d.pop().unwrap(), tmp]);
        } else {
            tmp.extend(d.pop().unwrap());
            e.push(tmp);
        }
    }
    let mut f = vec![];
    while e.len() > 1 {
        let mut c1 = e.pop().unwrap();
        let c2 = e.pop().unwrap();
        if query(c1[0], c2[0]) {
            c.push([c1, c2]);
        } else {
            c1.extend(c2);
            f.push(c1);
        }
    }
    if e.len() == 1 {
        let mut tmp = f.pop().unwrap();
        if query(e[0][0], tmp[0]) {
            c.push([e.pop().unwrap(), tmp]);
        } else {
            tmp.extend(e.pop().unwrap());
            f.push(tmp);
        }
    }
    let k = f[0][0];
    let mut ans = vec![];
    for [a, b] in &c {
        if query(k, a[0]) {
            ans.extend(a.iter().copied());
        } else {
            ans.extend(b.iter().copied());
        }
    }
    print!("! ");
    for &x in &ans {
        print!("{} ", x + 1);
    }
    println!();
}
