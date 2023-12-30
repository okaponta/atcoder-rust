use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut s:Chars,
    }
    s.reverse();
    let mut ans = vec![];
    let mut i = 0;
    while i != n {
        for j in (0..=m).rev() {
            if j == 0 {
                println!("-1");
                return;
            }
            if n < i + j {
                continue;
            }
            if s[i + j] == '0' {
                ans.push(j);
                i += j;
                break;
            }
        }
    }
    println!("{}", ans.iter().rev().join(" "));
}
