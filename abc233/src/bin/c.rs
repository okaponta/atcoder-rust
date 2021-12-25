use proconio::input;

fn main() {
    input! {
       n:usize,x:usize,
       a:[[usize];n]
    }
    println!("{}", dfs(0, 1, x, &a));
}

fn dfs(i: usize, p: usize, x: usize, a: &Vec<Vec<usize>>) -> usize {
    if i == a.len() {
        return if p == x { 1 } else { 0 };
    }
    let mut res = 0;
    for ai in &a[i] {
        res += dfs(i + 1, p.saturating_mul(*ai), x, a);
    }
    res
}
