use proconio::{input, marker::Chars};

fn main() {
    input! {
       n:usize,
       s:Chars,
       x:Chars,
    }
    let map = (0..7).into_iter().map(|i| (10 * i) % 7).collect::<Vec<_>>();
    let mut dp = vec![false; 7];
    dp[0] = true;
    for i in (0..n).rev() {
        let mut next = vec![false; 7];
        let num = s[i].to_digit(10).unwrap() as usize;
        for j in 0..7 {
            if x[i] == 'T' && (dp[map[j]] || dp[(map[j] + num) % 7]) {
                next[j] = true;
            }
            if x[i] == 'A' && dp[map[j]] && dp[(map[j] + num) % 7] {
                next[j] = true;
            }
        }
        dp = next;
    }
    println!("{}", if dp[0] { "Takahashi" } else { "Aoki" });
}
