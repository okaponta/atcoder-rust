use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut ans = 0;
    let mut o = 0;
    let mut x = 0;
    let mut r = 0;
    if s[r] == 'o' {
        o += 1;
    } else {
        x += 1;
    }
    for l in 0..n {
        while r < n && (o == 0 || x == 0) {
            r += 1;
            if r == n {
                break;
            }
            if s[r] == 'o' {
                o += 1;
            } else {
                x += 1;
            }
        }
        ans += n - r;
        if s[l] == 'o' {
            o -= 1;
        } else {
            x -= 1;
        }
    }
    println!("{}", ans);
}
