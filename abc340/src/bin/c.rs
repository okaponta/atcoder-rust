use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut memo = HashMap::new();
    memo.insert(1, 0);
    println!("{}", dfs(n, &mut memo));
}

fn dfs(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if memo.contains_key(&n) {
        return memo[&n];
    }
    let mut res = n;
    if n % 2 == 0 {
        res += 2 * dfs(n / 2, memo);
    } else {
        res += dfs(n / 2, memo);
        res += dfs((n / 2) + 1, memo);
    }
    memo.insert(n, res);
    res
}
