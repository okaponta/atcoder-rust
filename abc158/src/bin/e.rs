use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        p:usize,
        s:Chars,
    }
    let mut ans = 0;
    let mut dp = vec![0usize; p];
    dp[0] += 1;
    if p == 2 || p == 5 {
        for i in 0..n {
            let c = (s[i] as u8 - b'0') as usize;
            if c % p == 0 {
                ans += i + 1;
            }
        }
        println!("{}", ans);
        return;
    }
    let mut ten = 1;
    let mut tmp = 0;
    for i in (0..n).rev() {
        let c = (s[i] as u8 - b'0') as usize;
        tmp = (tmp + c * ten) % p;
        ans += dp[tmp];
        dp[tmp] += 1;
        ten = (ten * 10) % p;
    }
    println!("{}", ans);
}
