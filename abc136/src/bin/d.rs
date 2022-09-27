use itertools::{self, Itertools};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s:Chars,
    }
    let n = s.len();
    let mut ans = vec![0; n];
    s.push('R');
    let mut prev = 'R';
    let mut rl_count = vec![0; 2];
    let mut rl_idx = vec![0; 2];
    for i in 0..=n {
        if s[i] == 'R' {
            if prev == 'L' {
                ans[rl_idx[0]] = rl_count[0] - rl_count[0] / 2 + rl_count[1] / 2;
                ans[rl_idx[1]] = rl_count[0] / 2 + rl_count[1] - rl_count[1] / 2;
                rl_count = vec![0; 2];
            }
            rl_count[0] += 1;
        } else {
            if prev == 'R' {
                rl_idx[0] = i - 1;
                rl_idx[1] = i;
            }
            rl_count[1] += 1;
        }
        prev = s[i];
    }
    println!("{}", ans.iter().join(" "));
}
