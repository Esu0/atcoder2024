use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 3],
    }

    let mut count = vec![0; n];
    let mut indice = vec![(usize::MAX, usize::MAX); n];
    for (i, &ai) in a.iter().enumerate() {
        count[ai - 1] += 1;
        if count[ai - 1] == 2 {
            indice[ai - 1] = (i, ai);
        }
    }

    indice.sort_unstable_by_key(|&(i, _)| i);
    for (_, ai) in indice {
        print!("{} ", ai);
    }
}
