use std::num::NonZeroUsize;

use proconio::{input, marker};

struct Node {
    parent: usize,
    children: [Option<NonZeroUsize>; 26],
    is_terminal: bool,
}
impl Node {
    const fn new(parent: usize) -> Self {
        Self {
            parent,
            children: [None; 26],
            is_terminal: false,
        }
    }
}

fn main() {
    input! {
        n: usize,
        s: [marker::Bytes; n],
    }
    let mut trie = vec![Node::new(usize::MAX)];
    trie[0].is_terminal = true;
    let insert = |trie: &mut Vec<Node>, s: &[u8]| {
        let mut node = 0;
        for &c in s {
            let c = (c - b'a') as usize;
            if let Some(next) = trie[node].children[c] {
                node = next.get();
            } else {
                let next = NonZeroUsize::new(trie.len()).unwrap();
                trie.push(Node::new(node));
                trie[node].children[c] = Some(next);
                node = next.get();
            }
        }
        trie[node].is_terminal = true;
        node
    };

    let mut dp = vec![0];
    for si in &s {
        let mut ans = si.len();
        let mut node = 0;
        for (j, &cj) in si.iter().enumerate() {
            let c = (cj - b'a') as usize;
            if let Some(next) = trie[node].children[c] {
                node = next.get();
            } else {
                break;
            }
            ans = ans.min(dp[node] + (si.len() - j - 1));
        }
        println!("{}", ans);
        let mut node = insert(&mut trie, si);
        dp.resize(trie.len(), usize::MAX);
        dp[node] = 0;
        loop {
            let parent = trie[node].parent;
            if parent == usize::MAX {
                break;
            }
            dp[parent] = dp[parent].min(dp[node] + 1);
            node = parent;
        }
    }
}
