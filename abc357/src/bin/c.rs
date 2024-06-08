use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:u32,
    }
    let m = 3usize.pow(n);
    let mut ans = vec![vec!['.'; m]; m];
    f(n as usize, 0, 0, m / 3, &mut ans);
    for i in 0..m {
        println!("{}", ans[i].iter().join(""));
    }
}

fn f(k: usize, i1: usize, j1: usize, m: usize, ans: &mut Vec<Vec<char>>) {
    if k == 0 {
        ans[i1][j1] = '#';
        return;
    }
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            f(k - 1, i1 + i * m, j1 + j * m, m / 3, ans);
        }
    }
}
