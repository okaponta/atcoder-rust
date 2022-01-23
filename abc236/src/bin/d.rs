use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:usize,
    }
    let mut a = vec![vec![0; 2 * n]; 2 * n];
    for i in 0..2 * n {
        for j in i + 1..2 * n {
            input! {x: usize}
            a[i][j] = x;
        }
    }
    let ans = dfs(&a, 0, (0..2 * n).collect_vec());
    println!("{}", ans);
}

fn dfs(a: &Vec<Vec<usize>>, sum: usize, rem: Vec<usize>) -> usize {
    if rem.is_empty() {
        return sum;
    }
    let mut ans = 0;
    for i in 1..rem.len() {
        let n_sum = a[rem[0]][rem[i]] ^ sum;
        let mut n_rem = rem.clone();
        n_rem.remove(i);
        n_rem.remove(0);
        ans = ans.max(dfs(a, n_sum, n_rem));
    }
    ans
}
