use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        s:usize,
        ab:[(usize,usize);n],
    }
    let mut dp = vec![None; s + 1];
    dp[0] = Some(vec![]);
    for i in 0..n {
        let (a, b) = ab[i];
        let mut next = vec![None; s + 1];
        for j in 0..=s {
            if let Some(v) = &dp[j] {
                if j + a <= s && next[j + a].is_none() {
                    let mut va = v.clone();
                    va.push(true);
                    next[j + a] = Some(va);
                }
                if j + b <= s && next[j + b].is_none() {
                    let mut vb = v.clone();
                    vb.push(false);
                    next[j + b] = Some(vb);
                }
            }
        }
        dp = next;
    }
    if let Some(v) = &dp[s] {
        println!(
            "{}",
            v.into_iter().map(|b| if *b { 'A' } else { 'B' }).join("")
        );
    } else {
        println!("Impossible")
    }
}
