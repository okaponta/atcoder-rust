use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,k:usize
    }
    let mut cum = s
        .iter()
        .scan(0, |state, x| {
            if *x == '.' {
                *state += 1;
            }
            Some(*state)
        })
        .collect::<Vec<_>>();
    cum.insert(0, 0);
    let mut r = 0;
    let mut ans = 0;
    for l in 0..s.len() {
        while r < s.len() && cum[r + 1] - cum[l] <= k {
            r += 1;
        }
        ans = ans.max(r - l);
    }
    println!("{}", ans);
}
