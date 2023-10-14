use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        t:Chars,
        s:[Chars;n],
    }
    let mut head = vec![];
    let mut tail = vec![];
    for i in 0..n {
        let mut k = 0;
        for j in 0..s[i].len() {
            if k == t.len() {
                break;
            }
            if s[i][j] == t[k] {
                k += 1;
            }
        }
        head.push(k);
        k = 0;
        for j in (0..s[i].len()).rev() {
            if k == t.len() {
                break;
            }
            if s[i][j] == t[t.len() - 1 - k] {
                k += 1;
            }
        }
        tail.push(k);
    }
    tail.sort();
    let mut ans = 0;
    for i in head {
        ans += tail.len() - tail.lower_bound(&(t.len() - i));
    }
    println!("{}", ans);
}
