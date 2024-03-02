use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[[usize;n];n],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                g[i].push(j + 1);
            }
        }
    }
    for i in 0..n {
        println!("{}", g[i].iter().join(" "));
    }
}
