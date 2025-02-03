use proconio::*;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let mut s = vec![0; n + 1];
    let mut tmp = 0;
    for i in (0..n).rev() {
        tmp ^= a[i];
        s[i] = tmp;
    }
    println!("{}", dfs(0, 0, 0, n, k, &a, &s));
}

fn dfs(
    i: usize,
    j: usize,
    xor: usize,
    n: usize,
    k: usize,
    a: &Vec<usize>,
    s: &Vec<usize>,
) -> usize {
    let mut res = 0;
    if j == k {
        return xor;
    }
    if n - i == k - j {
        return xor ^ s[i];
    }
    res = res.max(dfs(i + 1, j + 1, xor ^ a[i], n, k, a, s));
    res = res.max(dfs(i + 1, j, xor, n, k, a, s));
    res
}
