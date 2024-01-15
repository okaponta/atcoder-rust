use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars
    }
    let cv = |c: &char| *c as usize - '0' as usize;
    let v = n.iter().map(cv).collect::<Vec<_>>();
    let mut ans = 0;
    for sum in 1..127 {
        let mut dp = vec![vec![vec![0usize; 2]; 127]; 127];
        dp[0][0][1] = 1;
        for i in 0..v.len() {
            let mut next = vec![vec![vec![0usize; 2]; 127]; 127];
            for ketawa in 0..sum + 1 {
                for rem in 0..sum {
                    for f in 0..2 {
                        for digit in 0..10 {
                            let n_keta = ketawa + digit;
                            let n_rem = (rem * 10 + digit) % sum;
                            let mut nf = f;
                            if n_keta > sum {
                                continue;
                            }
                            if f == 1 {
                                if v[i] < digit {
                                    continue;
                                }
                                if v[i] > digit {
                                    nf = 0
                                }
                            }
                            next[n_keta][n_rem][nf] += dp[ketawa][rem][f];
                        }
                    }
                }
            }
            dp = next;
        }
        ans += dp[sum][0][1];
        ans += dp[sum][0][0];
    }
    println!("{}", ans);
}
