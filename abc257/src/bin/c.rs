use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        s:Chars,
        w:[usize;n],
    }
    let mut p = vec![];
    let mut ww = vec![];
    for i in 0..n {
        p.push((w[i], if s[i] == '1' { 1 } else { 0 }));
        ww.push(w[i]);
    }
    p.sort();
    // 体重,大人なら1
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] + p[i].1);
    }
    ww.sort();
    ww.dedup();

    let mut ans = n - s[n];
    for wi in ww {
        let h = p.lower_bound(&(wi, 0));
        let temp = (h - s[h]) + (s[n] - s[h]);
        ans = ans.max(temp);
    }
    println!("{}", ans);
}
