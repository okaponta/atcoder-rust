use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut del = false;
    let mut ans = vec![];
    for c in s {
        if c == '|' {
            del = !del;
            continue;
        }
        if !del {
            ans.push(c);
        }
    }
    println!("{}", ans.iter().join(""));
}
