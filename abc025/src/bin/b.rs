use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        a:i64,
        b:i64,
        sd:[(Chars,i64);n],
    }
    let mut ans = 0;
    for (s, d) in sd {
        if s[0] == 'E' {
            ans += f(d, a, b);
        } else {
            ans -= f(d, a, b);
        }
    }
    if 0 < ans {
        println!("East {}", ans);
    } else if ans < 0 {
        println!("West {}", -ans);
    } else {
        println!("0");
    }
}

fn f(d: i64, a: i64, b: i64) -> i64 {
    d.max(a).min(b)
}
