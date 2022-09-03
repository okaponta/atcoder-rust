use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = 1;
    if s[1] == 'o' {
        ans = 5;
    }
    if s[1] == 'u' {
        ans = 4;
    }
    if s[1] == 'e' {
        ans = 3;
    }
    if s[1] == 'h' {
        ans = 2;
    }
    println!("{}", ans);
}
