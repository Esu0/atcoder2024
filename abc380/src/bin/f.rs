use std::collections::HashMap;

use proconio::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Res {
    Win,
    Lose,
}

fn solve(
    state: usize,
    takahashi: bool,
    cards: &[u32],
    memo: &mut HashMap<(usize, bool), Res>,
) -> Res {
    if let Some(res) = memo.get(&(state, takahashi)) {
        return *res;
    }
    let orig_state = state;
    for i in 0..cards.len() {
        let mut state = orig_state;
        if ((state >> (2 * i)) & 3) == (if takahashi { 0 } else { 1 }) {
            state &= !(0b11 << (2 * i));
            state |= 2 << (2 * i);
            if let Res::Lose = solve(state, !takahashi, cards, memo) {
                memo.insert((orig_state, takahashi), Res::Win);
                return Res::Win;
            }
            for (j, &cardj) in cards.iter().enumerate() {
                let mut state = state;
                if ((state >> (2 * j)) & 3) == 2 && cards[i] > cardj {
                    state &= !(0b11 << (2 * j));
                    state |= (if takahashi { 0 } else { 1 }) << (2 * j);
                    if let Res::Lose = solve(state, !takahashi, cards, memo) {
                        memo.insert((orig_state, takahashi), Res::Win);
                        return Res::Win;
                    }
                }
            }
        }
    }
    memo.insert((orig_state, takahashi), Res::Lose);
    Res::Lose
}

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        cards: [u32; n + m + l],
    }
    // 0ならTakahashiの手札、1ならAokiの手札、2なら場札
    let mut start = 0;
    let takahashi = true;
    let mut i = n;
    for _ in 0..m {
        start |= 1 << (i * 2);
        i += 1;
    }
    for _ in 0..l {
        start |= 2 << (i * 2);
        i += 1;
    }
    // eprintln!("{:08b}", start);
    let mut memo = HashMap::new();
    // assert!(memo.keys().all(|&(s, _)| (0..n).all(|i| {
    //     (s >> (i * 2)) & 0b11 != 3
    // }) && s & !((1 << ((n + m + l) * 2)) - 1) == 0));
    let res = solve(start, takahashi, &cards, &mut memo);
    // memo.iter().for_each(|(&k, &v)| {
    //     eprintln!("{:08b} {} {:?}", k.0, k.1, v)
    // });
    if let Res::Win = res {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
