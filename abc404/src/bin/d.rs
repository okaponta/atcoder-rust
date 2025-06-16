#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        c:[usize;n],
    }
    let mut zoo = vec![vec![]; n];
    for i in 0..m {
        input! {
            k:usize,
            a:[Usize1;k],
        }
        for a in a {
            zoo[a].push(i);
        }
    }
    println!("{}", dfs(0, &mut vec![0; m], &zoo, &c, 0, n));
}

fn dfs(
    fee: usize,
    cnt: &mut Vec<usize>,
    zoo: &Vec<Vec<usize>>,
    c: &Vec<usize>,
    cur: usize,
    n: usize,
) -> usize {
    let mut res = 1 << 60;
    if cur == n {
        if cnt.iter().all(|&x| x >= 2) {
            res = res.min(fee);
        }
        return res;
    }
    res = res.min(dfs(fee, cnt, zoo, c, cur + 1, n));
    for &z in &zoo[cur] {
        cnt[z] += 1;
    }
    res = res.min(dfs(fee + c[cur], cnt, zoo, c, cur + 1, n));
    for &z in &zoo[cur] {
        cnt[z] += 1;
    }
    res = res.min(dfs(fee + 2 * c[cur], cnt, zoo, c, cur + 1, n));
    for &z in &zoo[cur] {
        cnt[z] -= 2;
    }
    res
}
