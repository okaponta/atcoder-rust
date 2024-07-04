use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        t:i64,
        s:Chars,
        x:[i64;n],
    }
    let mut plus = vec![];
    let mut minus = vec![];
    for i in 0..n {
        if s[i] == '0' {
            minus.push(x[i]);
        } else {
            plus.push(x[i]);
        }
    }
    minus.sort();
    let mut ans = 0;
    for i in plus {
        let bef = minus.upper_bound(&i);
        let after = minus.upper_bound(&(i + 2 * t));
        ans += after - bef;
    }
    println!("{}", ans);
}
