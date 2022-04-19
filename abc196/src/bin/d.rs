use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       h:i32,w:i32,
       a:i32,_:i32,
    }
    let ans = dfs(h, w, a, 1, 0, HashSet::new());
    println!("{}", ans);
}

fn dfs(h: i32, w: i32, a: i32, cur: i32, united: i32, used: HashSet<i32>) -> usize {
    if united == a {
        return 1;
    }
    if cur == h * w {
        return 0;
    }
    let mut count = 0;
    // 結合しない場合
    count += dfs(h, w, a, cur + 1, united, used.clone());
    // 結合する場合
    for n in next(h, w, cur) {
        if used.contains(&cur) || used.contains(&n) {
            continue;
        }
        let mut new_used = used.clone();
        new_used.insert(cur);
        new_used.insert(n);
        count += dfs(h, w, a, cur + 1, united + 1, new_used);
    }
    count
}

fn next(h: i32, w: i32, cur: i32) -> Vec<i32> {
    let mut res = vec![];
    if cur == h * w {
        return res;
    }
    if cur % w != 0 {
        res.push(cur + 1);
    }
    if cur + w <= h * w {
        res.push(cur + w);
    }
    res
}
