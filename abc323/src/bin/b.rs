use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut win = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                win[i] += 1;
            }
        }
    }
    let mut ans = vec![];
    for i in (0..n).rev() {
        for j in 0..n {
            if win[j] == i {
                ans.push(j + 1);
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}
