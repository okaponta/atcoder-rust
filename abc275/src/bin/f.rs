use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;n],
    }
    a.insert(0, 0);
    let mut dp = vec![(5000, 0); m + 1];
    dp[0] = (0, 5000);
    for i in 1..=n {
        let mut next = vec![(5000, 0); m + 1];
        for j in 0..=m {
            let (x, y) = dp[j];
            // 消す
            let t1 = if y + 1 == i { x } else { x + 1 };
            if t1 <= next[j].0 {
                next[j] = (t1, i);
            }
            // 消さない
            if m < j + a[i] {
                continue;
            }
            if x <= next[j + a[i]].0 {
                next[j + a[i]] = (x, y);
            }
        }
        dp = next;
    }
    for i in 1..=m {
        if dp[i].0 == 5000 {
            println!("-1");
            continue;
        }
        println!("{}", dp[i].0);
    }
}
