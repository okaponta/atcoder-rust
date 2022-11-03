use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let atcg = vec!['A', 'T', 'C', 'G'].into_iter().collect::<HashSet<_>>();
    let mut ans = 0;
    let mut temp = 0;
    for c in s {
        if atcg.contains(&c) {
            temp += 1;
        } else {
            ans = ans.max(temp);
            temp = 0;
        }
    }
    println!("{}", ans.max(temp));
}
