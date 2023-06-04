use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        a:usize,
        b:usize,
        q:usize,
        s:[i64;a],
        t:[i64;b],
        x:[i64;q],
    }
    for x in x {
        let mut ans = 1 << 60;
        let sl = left(x, &s);
        let sr = right(x, &s);
        let tl = left(x, &t);
        let tr = right(x, &t);
        // 左左
        ans = ans.min(x - left(sl, &t));
        ans = ans.min(x - left(tl, &s));
        // 右右
        ans = ans.min(right(sr, &t) - x);
        ans = ans.min(right(tr, &s) - x);
        // 左右
        ans = ans.min(x - sl + right(sl, &t) - sl);
        ans = ans.min(x - tl + right(tl, &s) - tl);
        // 右左
        ans = ans.min(sr - x + sr - left(sr, &t));
        ans = ans.min(tr - x + tr - left(tr, &s));
        println!("{}", ans);
    }
}

fn left(x: i64, s: &Vec<i64>) -> i64 {
    let pos = s.lower_bound(&x);
    if pos == 0 {
        return -1 << 60;
    }
    s[pos - 1]
}

fn right(x: i64, s: &Vec<i64>) -> i64 {
    let pos = s.lower_bound(&x);
    if pos == s.len() {
        return 1 << 60;
    }
    s[pos]
}
