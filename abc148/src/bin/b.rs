use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
        t:Chars,
    }
    let mut ans = vec![];
    for i in 0..n {
        ans.push(s[i]);
        ans.push(t[i]);
    }
    println!("{}", ans.iter().collect::<String>());
}
