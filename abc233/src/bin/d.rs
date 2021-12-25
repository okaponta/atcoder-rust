use std::collections::HashMap;

use proconio::input;

fn main() {
    // TLE
    input! {
       n:i64,k:i64,
       a:[i64;n]
    }
    let mut s = a
        .iter()
        .scan(0, |state, &x| {
            *state = *state + x;
            Some(*state)
        })
        .collect::<Vec<_>>();
    s.insert(0, 0);
    let mut counts: HashMap<i64, Vec<usize>> = HashMap::new();
    for i in 0..s.len() {
        if counts.contains_key(&s[i]) {
            let mut after = counts.get(&s[i]).unwrap().clone();
            after.push(i);
            counts.insert(s[i], after);
        } else {
            counts.insert(s[i], vec![i]);
        }
    }
    let mut ans = 0;
    for i in 0..s.len() {
        let target = s[i] + k;
        if counts.contains_key(&target) {
            let cand = counts.get(&target).unwrap();
            for c in cand {
                if c > &i {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
