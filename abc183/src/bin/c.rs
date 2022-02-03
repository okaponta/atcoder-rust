use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,k:usize,
        t:[[usize;n];n],
    }
    println!("{}", dfs(0, (1..n).collect_vec(), &t, 0, k, 0));
}

fn dfs(
    cur: usize,
    remain: Vec<usize>,
    t: &Vec<Vec<usize>>,
    mut time: usize,
    k: usize,
    mut count: usize,
) -> usize {
    if remain.is_empty() {
        time += t[cur][0];
        return if time == k { count + 1 } else { count };
    }
    for i in 0..remain.len() {
        let next = remain[i];
        let mut next_rem = remain.clone();
        next_rem.remove(i);
        count = dfs(next, next_rem, t, time + t[cur][next], k, count);
    }
    return count;
}
