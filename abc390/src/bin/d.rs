use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut set = HashSet::new();
    dfs(0, n, &a, &mut vec![], &mut set);
    println!("{}", set.len());
}

fn dfs(i: usize, n: usize, a: &Vec<usize>, xor: &mut Vec<usize>, set: &mut HashSet<usize>) {
    if i == n {
        set.insert(xor.iter().fold(0, |s, &x| s ^ x));
        return;
    }
    for j in 0..xor.len() {
        xor[j] += a[i];
        dfs(i + 1, n, a, xor, set);
        xor[j] -= a[i];
    }
    xor.push(a[i]);
    dfs(i + 1, n, a, xor, set);
    xor.pop();
}
