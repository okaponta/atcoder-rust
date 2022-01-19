use proconio::input;

const MAX: usize = 100001;

fn main() {
    input! {
       n:usize,
       t:[usize;n]
    }

    let mut dp = vec![false; MAX];
    dp[0] = true;
    // dp[i] = iが組み合わせとして存在するかどうか
    let sum = t.clone().iter().sum::<usize>();
    for time in t {
        for i in (0..MAX - 1).rev() {
            if dp[i] {
                dp[i + time] = true;
            }
        }
    }
    for i in (sum + 1) / 2..MAX - 1 {
        if dp[i] {
            println!("{}", i);
            return;
        }
    }
}
