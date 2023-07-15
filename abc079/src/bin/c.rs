use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let a = s[0].to_digit(10).unwrap() as i32;
    let b = s[1].to_digit(10).unwrap() as i32;
    let c = s[2].to_digit(10).unwrap() as i32;
    let d = s[3].to_digit(10).unwrap() as i32;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let mut ans = a;
                if i == 0 {
                    ans += b;
                } else {
                    ans -= b;
                }
                if j == 0 {
                    ans += c;
                } else {
                    ans -= c;
                }
                if k == 0 {
                    ans += d;
                } else {
                    ans -= d;
                }
                if ans == 7 {
                    println!(
                        "{}{}{}{}{}{}{}=7",
                        a,
                        if i == 0 { '+' } else { '-' },
                        b,
                        if j == 0 { '+' } else { '-' },
                        c,
                        if k == 0 { '+' } else { '-' },
                        d
                    );
                    return;
                }
            }
        }
    }
}
