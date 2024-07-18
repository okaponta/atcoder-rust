use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = vec![];
    for c in s {
        if c == 'B' {
            ans.pop();
        } else {
            ans.push(c);
        }
    }
    println!("{}", ans.iter().join(""));
}
