use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n:usize,
        s:Chars,
    }
    let mut ans = 0i64;
    let mut tmp = 1;
    for c in s {
        if c == 'b' {
            ans += tmp;
        }
        if c == 'c' {
            ans += tmp * 2;
        }
        tmp *= 2;
    }
    println!("{}", ans);
}
