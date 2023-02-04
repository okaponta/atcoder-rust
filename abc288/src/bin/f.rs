use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        x:Chars,
    }
    let m = 998244353;
    let mut ans = cv(x[0]);
    let mut sum = 1 + cv(x[0]);
    for i in 1..n {
        ans = (10 * ans + sum * cv(x[i])) % m;
        sum = (sum + ans) % m;
    }
    println!("{}", ans);
}

fn cv(c: char) -> usize {
    c.to_digit(10).unwrap() as usize
}
