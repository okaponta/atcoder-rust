use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[i64;n],
    }
    let max = n as i64;
    let mut ans = vec![vec![]; m];
    for i in 0..n {
        let m = m as i64;
        let diff = (i + 1) as i64;
        let first = a[i] + diff;
        let last = a[i] + diff * m;
        // 一回の操作で対象外
        if max < first {
            continue;
        }
        // m回操作しても対象外
        if last < 0 {
            continue;
        }
        let start = 0.max((-first + diff - 1) / diff);
        let end = ((n as i64 - first + diff - 1) / diff).min(m - 1);
        for j in start..=end {
            ans[j as usize].push(a[i] + diff * (j + 1) as i64);
        }
    }
    for mut ans in ans {
        if ans.len() == 0 {
            println!("0");
            continue;
        }
        ans.sort();
        ans.dedup();
        let mut mex = 0;
        while ans[mex] == mex as i64 {
            mex += 1;
            if mex == ans.len() {
                break;
            }
        }
        println!("{}", mex);
    }
}
