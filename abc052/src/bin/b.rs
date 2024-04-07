use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n:usize,
        s:Chars,
    }
    let mut ans = 0;
    let mut x = 0;
    for c in s {
        if c == 'I' {
            x += 1;
        } else {
            x -= 1;
        }
        ans = ans.max(x);
    }
    println!("{}", ans);
}
