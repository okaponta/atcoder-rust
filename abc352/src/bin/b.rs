use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let mut ans = vec![];
    let mut i = 0;
    for j in 0..s.len() {
        while s[j] != t[i] {
            i += 1;
        }
        ans.push(i + 1);
        i += 1;
    }
    println!("{}", ans.iter().join(" "));
}
