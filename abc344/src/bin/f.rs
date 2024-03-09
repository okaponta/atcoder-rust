use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[[usize;n];n],
        r:[[usize;n-1];n],
        d:[[usize;n];n-1],
    }
    let mut pp = vec![];
    for (i, j) in iproduct!(0..n, 0..n) {
        pp.push(p[i][j]);
    }
    let map = compress(&pp);
    let m = map.len();
    let mut rev = vec![0; m];
    for (&k, &v) in map.iter() {
        rev[v] = k;
    }

    let mut dp = vec![vec![vec![(1 << 60, 0); m]; n]; n];
    dp[0][0][map[&p[0][0]]] = (0, 0);
    for (i, j, k) in iproduct!(0..n, 0..n, 0..m) {
        let (times, money) = dp[i][j][k];
        let nk = k.max(map[&p[i][j]]);
        if i != n - 1 {
            if d[i][j] <= money {
                dp[i + 1][j][nk] = compare(dp[i + 1][j][nk], (times + 1, money - d[i][j]));
            } else {
                let tmpt = (d[i][j] - money + rev[nk] - 1) / rev[nk];
                dp[i + 1][j][nk] = compare(
                    dp[i + 1][j][nk],
                    (times + tmpt + 1, money + tmpt * rev[nk] - d[i][j]),
                );
            }
        }
        if j != n - 1 {
            if r[i][j] <= money {
                dp[i][j + 1][nk] = compare(dp[i][j + 1][nk], (times + 1, money - r[i][j]));
            } else {
                let tmpt = (r[i][j] - money + rev[nk] - 1) / rev[nk];
                dp[i][j + 1][nk] = compare(
                    dp[i][j + 1][nk],
                    (times + tmpt + 1, money + tmpt * rev[nk] - r[i][j]),
                );
            }
        }
    }

    let mut ans = 1 << 60;
    for i in 0..m {
        ans = ans.min(dp[n - 1][n - 1][i].0);
    }
    println!("{}", ans);
}

fn compare(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    if a.0 < b.0 {
        return a;
    }
    if b.0 < a.0 {
        return b;
    }
    if b.1 < a.1 {
        return a;
    }
    b
}

fn compress(source: &[usize]) -> HashMap<usize, usize> {
    let set: HashSet<&usize> = source.iter().collect();
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(*x, i);
    }
    result
}
