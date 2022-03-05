use proconio::{input, marker::Chars};

fn main() {
    input! {
        t:i32,
    }
    for _ in 0..t {
        testcase();
    }
}

fn testcase() {
    input! {
        n:i32,
        s:Chars,
    }
    let mut dp = [1, 0];
    for i in 0..((n + 1) / 2) {
        let mut next = [0, 0];
        let num = (s[i as usize] as u8 - b'A') as i64;
        next[0] = dp[0];
        next[1] = dp[0] * num;
        next[1] += dp[1] * 26;
        next[0] %= 998244353;
        next[1] %= 998244353;
        dp = next;
    }
    let mut judge_mid = false;
    if n % 2 == 0 {
        let ss = s.iter().collect::<String>();
        let mut zenhan = ss.clone();
        let _ = zenhan.split_off(n as usize / 2);
        let rev: String = zenhan.clone().chars().rev().collect();
        let target = zenhan + &rev;
        let mut judge = vec![ss.clone(), target];
        judge.sort();
        if judge[0] == ss && judge[1] != ss {
            judge_mid = true;
        }
    } else {
        let ss = s.iter().collect::<String>();
        let mut zenhan = ss.clone();
        let _ = zenhan.split_off(n as usize / 2);
        let rev: String = zenhan.clone().chars().rev().collect();
        let mid = s.iter().nth(n as usize / 2).unwrap().to_string();
        let target = zenhan + &mid + &rev;
        let mut judge = vec![ss.clone(), target];
        judge.sort();
        if judge[0] == ss && judge[1] != ss {
            judge_mid = true;
        }
    }
    let ans = if judge_mid {
        dp[0] + dp[1] - 1
    } else {
        dp[0] + dp[1]
    };
    println!("{}", ans % 998244353);
}
