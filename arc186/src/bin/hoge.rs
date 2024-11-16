use std::collections::{HashMap, HashSet};

const N: usize = 4;
fn main() {
    let mut matricies = Vec::with_capacity(1 << (N * N));
    for b in 0u32..(1 << (N * N)) {
        matricies.push(std::array::from_fn::<_, N, _>(|i| ((b >> (i * N)) & ((1 << N) - 1)) as u8));
    }

    let mut map = HashMap::<_, Vec<_>>::new();
    for matrix in &matricies {
        let mut key = 0;
        for &row in matrix {
            key = key * (N + 1) + row.count_ones() as usize;
        }
        for j in 0..N {
            let mut count = 0;
            for &row in matrix {
                count += (row >> j) & 1;
            }
            key = key * (N + 1) + count as usize;
        }
        map.entry(key).and_modify(|elems| elems.push(*matrix)).or_insert_with(|| vec![*matrix]);
    }

    let mut answer_set = HashSet::new();
    for matrices in map.values() {
        let mut and = [(1u8 << N) - 1; N];
        let mut nor = [(1u8 << N) - 1; N];
        for matrix in matrices {
            for (&row, (andi, nori)) in matrix.iter().zip(and.iter_mut().zip(nor.iter_mut())) {
                *andi &= row;
                *nori &= (!row) & ((1 << N) - 1);
            }
        }
        let mut count = 0;
        for r in and {
            count += r.count_ones();
        }
        for r in nor {
            count += r.count_ones();
        }
        answer_set.insert(count);
    }
    eprintln!("{:?}", answer_set);
    println!("{} {}", N, N * N + 1);
    for i in 0..=N * N {
        if !answer_set.contains(&(i as u32)) {
            println!("{}", i);
        }
    }
    for i in 0..=N * N {
        if answer_set.contains(&(i as u32)) {
            println!("{}", i);
        }
    }
}
