use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
       n:i32,
       s:[i32;n]
    }
    let mut cand: HashSet<i32> = HashSet::new();
    for a in 1..200 {
        for b in 1..200 {
            let area = 4 * a * b + 3 * a + 3 * b;
            if area > 1000 {
                break;
            }
            cand.insert(area);
        }
    }
    let ans = s.iter().filter(|&x| !cand.contains(x)).count();
    println!("{}", ans);
}
