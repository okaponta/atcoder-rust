use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       n:i64,
    }
    let mut tka = vec![vec![]; n as usize];
    for i in 0..n {
        input! {
            t:i64,
            k:i64,
            a:[i64;k],
        }
        let mut tkai = a;
        tkai.insert(0, t);
        tkai.insert(1, k);

        tka[i as usize] = tkai;
    }
    tka.insert(0, vec![0]);

    let mut need_waza = vec![n];
    let mut mastered_waza = HashSet::new();
    let mut ans = 0;
    while need_waza.len() > 0 {
        let target = need_waza.pop().unwrap();
        if mastered_waza.contains(&target) {
            continue;
        }
        ans += tka[target as usize][0];
        mastered_waza.insert(target);
        let k = tka[target as usize][1];
        if k > 0 {
            for i in 2..=k + 1 {
                need_waza.push(tka[target as usize][i as usize]);
            }
        }
    }
    println!("{}", ans);
}
