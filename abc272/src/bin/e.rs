use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[i64;n],
    }
    let max = n as i64;
    let mut ans = vec![HashSet::new(); m];
    for i in 0..n {
        let m = m as i64;
        let diff = (i + 1) as i64;
        let first = a[i] + diff;
        let last = a[i] + diff * m;
        // 対象外
        if max < first || last < 0 {
            continue;
        }
        let start = 0.max((-first + diff - 1) / diff);
        let end = ((n as i64 - first + diff - 1) / diff).min(m - 1);
        for j in start..=end {
            ans[j as usize].insert(a[i] + diff * (j + 1) as i64);
        }
    }
    for ans in ans {
        let mut mex = 0;
        while ans.contains(&mex) {
            mex += 1;
        }
        println!("{}", mex);
    }
}
