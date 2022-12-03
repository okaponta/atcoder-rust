use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        _:usize,
        s:[Chars;h],
    }
    let mut ans = 0;
    for i in 0..h {
        for &c in &s[i] {
            if c == '#' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
