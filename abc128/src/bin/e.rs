use std::collections::BTreeSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut stx:[(usize,usize,usize);n],
        d:[usize;q],
    }
    let mut set = BTreeSet::new();
    for i in 0..q {
        set.insert((d[i], i));
    }
    let mut ans = vec![-1; q];
    stx.sort_by_key(|k| k.2);
    for (s, t, x) in stx {
        let from = s.saturating_sub(x);
        let to = t.saturating_sub(x);
        while let Some(&(d, i)) = set.range((from, 0)..(to, 0)).next() {
            set.remove(&(d, i));
            ans[i] = x as i64;
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
