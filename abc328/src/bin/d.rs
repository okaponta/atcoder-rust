use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = vec![];
    for c in s {
        ans.push(c);
        let n = ans.len();
        if 3 <= n && ans[n - 3] == 'A' && ans[n - 2] == 'B' && ans[n - 1] == 'C' {
            ans.pop();
            ans.pop();
            ans.pop();
        }
    }
    println!("{}", ans.iter().join(""));
}
