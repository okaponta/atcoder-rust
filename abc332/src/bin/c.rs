use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        m:usize,
        s:Chars,
    }
    let mut ans = 0;
    let mut t = vec![m, 0];
    for c in s {
        if c == '0' {
            t[0] = m;
            t[1] = ans;
        } else if c == '1' {
            if 0 < t[0] {
                t[0] -= 1;
            } else {
                if 0 < t[1] {
                    t[1] -= 1;
                } else {
                    ans += 1;
                }
            }
        } else {
            if 0 < t[1] {
                t[1] -= 1;
            } else {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
