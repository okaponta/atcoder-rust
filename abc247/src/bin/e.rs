use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize, y:usize,
        a:[usize;n],
    }
    if x == y {
        let mut before = 0;
        let mut ans: i64 = 0;
        for ai in a {
            let mut next = 0;
            if ai < y || x < ai {
                //範囲外
            }
            if y == ai {
                // min
                next = before + 1;
            }
            ans += next;
            before = next;
        }
        println!("{}", ans);
        return;
    }
    let mut dp = vec![0; 4];
    let mut ans: i64 = 0;
    for ai in a {
        let mut next = vec![0; 4];
        if ai < y || x < ai {
            //範囲外
        } else if y < ai && ai < x {
            // 範囲内
            next[0] += dp[0] + 1;
            for j in 1..4 {
                if dp[j] != 0 {
                    next[j] += dp[j];
                }
            }
        }
        if y == ai {
            // min
            next[1] += dp[0] + dp[1] + 1;
            next[3] += dp[2] + dp[3];
        }
        if x == ai {
            //max
            next[2] += dp[0] + dp[2] + 1;
            next[3] += dp[1] + dp[3];
        }
        ans += dp[3];
        dp = next;
    }
    println!("{}", ans + dp[3]);
}
