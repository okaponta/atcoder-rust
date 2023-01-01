use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    let mut i = 0;
    let mut ans = 0;
    while i < n {
        if i < n - 1 && s[i] == '0' && s[i + 1] == '0' {
            i += 2;
        } else {
            i += 1;
        }
        ans += 1;
    }
    println!("{}", ans);
}
