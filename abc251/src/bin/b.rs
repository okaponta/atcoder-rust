use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,w:usize,
        a:[usize;n],
    }
    let mut ans = vec![false; w + 1];
    // 1つ
    for ai in &a {
        if ai <= &w {
            ans[*ai] = true;
        }
    }
    // 2つ
    for c in a.iter().combinations(2) {
        let s = c[0] + c[1];
        if s <= w {
            ans[s] = true;
        }
    }
    // 3つ
    for c in a.iter().combinations(3) {
        let s = c[0] + c[1] + c[2];
        if s <= w {
            ans[s] = true;
        }
    }

    println!("{}", ans.iter().filter(|b| **b).count());
}
