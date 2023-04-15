use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut ok = 0;
    let mut ng = 0;
    for i in 0..n {
        if s[i] == 'o' {
            ok += 1;
        }
        if s[i] == 'x' {
            ng += 1;
        }
    }
    println!("{}", if 0 < ok && ng == 0 { "Yes" } else { "No" });
}
