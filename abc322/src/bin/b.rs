use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:Chars,
        t:Chars,
    }
    let mut ans = 3;
    if (0..n).into_iter().all(|i| s[i] == t[i]) {
        ans -= 2;
    }
    if (0..n).into_iter().all(|i| s[i] == t[m - n + i]) {
        ans -= 1;
    }
    println!("{}", ans);
}
