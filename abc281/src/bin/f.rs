use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n]
    }
    a.sort();
    a.dedup();
    println!("{}", dfs(&a, 0, a.len(), 1 << 30));
}

fn dfs(a: &Vec<usize>, l: usize, r: usize, bit: usize) -> usize {
    if l + 1 == r {
        return 0;
    }
    if (l..r).into_iter().all(|i| a[i] & bit == bit) {
        return dfs(a, l, r, bit >> 1);
    }
    if (l..r).into_iter().fold(0, |or, i| or | a[i]) & bit != bit {
        return dfs(a, l, r, bit >> 1);
    }
    let mid = (l..r).into_iter().find(|&i| a[i] & bit == bit).unwrap();
    return bit + dfs(a, l, mid, bit >> 1).min(dfs(a, mid, r, bit >> 1));
}
