use proconio::{input, marker::Usize1};

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize, m: usize, k: i64,
        a: [Usize1; m],
        uv: [(Usize1, Usize1); n-1]
    }
    let mut graph = vec![vec![]; n];
    for (i, &(u, v)) in uv.iter().enumerate() {
        graph[u].push((v, i));
        graph[v].push((u, i));
    }

    let mut count = vec![0; n - 1];
    for i in 0..m - 1 {
        dfs(&graph, &mut count, n + 1, a[i], a[i + 1]);
    }

    let s = count.iter().sum::<usize>() as i64;
    if (s + k) % 2 != 0 || s + k < 0 {
        println!("0");
        return;
    }

    let t = ((s + k) / 2) as usize;
    // dp[i] = いくつか選んだ和がiになる個数
    let mut dp = vec![0; t + 1];
    dp[0] = 1;
    for &ci in count.iter() {
        let mut next = dp.clone();
        for i in ci..=t {
            next[i] = (next[i] + dp[i - ci]) % MOD;
        }
        dp = next;
    }
    println!("{}", dp[t]);
}

fn dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    count: &mut Vec<usize>,
    prev: usize,
    current: usize,
    goal: usize,
) -> bool {
    if current == goal {
        return true;
    }
    for &(next, index) in graph[current].iter() {
        if next == prev {
            continue;
        }
        if dfs(graph, count, current, next, goal) {
            count[index] += 1;
            return true;
        }
    }
    false
}
