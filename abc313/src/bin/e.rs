use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut ans = 0;
    for i in (0..n - 1).rev() {
        let prev = s[i + 1].to_digit(10).unwrap() as usize;
        let cur = s[i].to_digit(10).unwrap() as usize;
        if 1 < cur && 1 < prev {
            println!("-1");
            return;
        }
        ans = (ans * prev + prev) % 998244353;
    }
    println!("{}", ans);
}
