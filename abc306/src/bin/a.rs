use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut ans = vec![];
    for i in 0..n {
        ans.push(s[i]);
        ans.push(s[i]);
    }
    println!("{}", ans.iter().collect::<String>());
}
