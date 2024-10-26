use proconio::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Query {
    Del(usize),
    Output(usize, usize),
}

macro_rules! chmin {
    ($d:expr, $s:expr) => {
        $d = std::cmp::min($d, $s);
    };
}


fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abc: [(usize, usize, u64); m],
    }

    let queries = (0..q).map(|_| {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! { x: usize }
                Query::Del(x - 1)
            }
            2 => {
                input! { u: usize, v: usize }
                Query::Output(u - 1, v - 1)
            }
            _ => unreachable!(),
        }
    }).collect::<Vec<_>>();

    let mut will_delete = vec![false; m];
    for &q in &queries {
        if let Query::Del(x) = q {
            will_delete[x] = true;
        }
    }

    let mut graph = vec![vec![u64::MAX; n]; n];
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        if !will_delete[i] {
            graph[a - 1][b - 1] = c;
            graph[b - 1][a - 1] = c;
        }
    }
    for (i, gi) in graph.iter_mut().enumerate() {
        gi[i] = 0;
    }

    // Warshall-Floyd
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                chmin!(graph[i][j], graph[i][k].saturating_add(graph[k][j]));
            }
        }
    }

    let mut ans = Vec::new();
    for &q in queries.iter().rev() {
        match q {
            Query::Del(i) => {
                let (a, b, c) = abc[i];
                let (a, b) = (a - 1, b - 1);
                for i in 0..n {
                    for j in 0..i {
                        chmin!(graph[i][j], graph[i][a].saturating_add(c).saturating_add(graph[b][j]));
                        chmin!(graph[i][j], graph[i][b].saturating_add(c).saturating_add(graph[a][j]));
                        graph[j][i] = graph[i][j];
                    }
                }
            }
            Query::Output(u, v) => {
                ans.push(graph[u][v] as i64);
            }
        }
    }
    for &a in ans.iter().rev() {
        println!("{}", a);
    }
}
