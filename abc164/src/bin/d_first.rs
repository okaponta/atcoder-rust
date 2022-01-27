use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = 0;
    let mut prev = vec![0i64; 2019];
    for c in s {
        let cint = c.to_digit(10).unwrap() as usize;
        let mut next = vec![0i64; 2019];
        for i in 0..2019 {
            if prev[i] == 0 {
                continue;
            } else {
                next[(i * 10 + cint) % 2019] += prev[i];
            }
        }
        next[cint] += 1;
        ans += next[0];
        prev = next;
    }
    println!("{}", ans);
}
