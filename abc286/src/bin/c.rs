use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        mut s:Chars,
    }
    let mut ans = 1 << 60;
    for i in 0..n {
        let mut tmp = a * i;
        for j in 0..n / 2 {
            if s[j] != s[n - 1 - j] {
                tmp += b;
            }
        }
        ans = ans.min(tmp);
        s.push(s[0]);
        s.remove(0);
    }
    println!("{}", ans);
}
