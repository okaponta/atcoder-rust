use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = -1;
    for i in 0..s.len() {
        if s[i] == 'a' {
            ans = (i + 1) as i32;
        }
    }
    println!("{}", ans);
}
