use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut ans = vec![vec![false; n]; n];
    for _ in 0..m {
        input! {
            k:usize,
            x:[Usize1;k],
        }
        for (&a, &b) in x.iter().tuple_combinations() {
            ans[a][b] = true;
        }
    }
    for i in 0..n {
        for j in i + 1..n {
            if !ans[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
