use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
       n:usize,
       mut m:usize,
       xy:[(usize,usize);m],
    }
    if n < m {
        m = n;
    }
    let mut ans = vec![false; 2 * m + 3];
    ans[m + 1] = true;
    let mut map = BTreeMap::new();
    for (x, y) in xy {
        if n - m <= y && y <= n + m {
            (*map.entry(x).or_insert(vec![])).push(y + m + 1 - n);
        }
    }
    for (_, v) in map {
        let mut f_list = vec![];
        let mut t_list = vec![];
        for p in v {
            if ans[p] && !(ans[p - 1] || ans[p + 1]) {
                f_list.push(p);
            }
            if !ans[p] && (ans[p - 1] || ans[p + 1]) {
                t_list.push(p);
            }
        }
        for f in f_list {
            ans[f] = false;
        }
        for t in t_list {
            ans[t] = true;
        }
    }
    println!("{}", ans.iter().filter(|&&b| b).count());
}
