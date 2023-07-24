use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:[Chars;2],
    }
    let ok = s[0][2] == s[1][0] && s[0][1] == s[1][1] && s[0][0] == s[1][2];
    println!("{}", if ok { "YES" } else { "NO" });
}
