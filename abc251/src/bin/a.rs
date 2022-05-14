use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = vec![];
    for i in 0..6 {
        ans.push(s[i % s.len()]);
    }
    println!("{}", ans.iter().collect::<String>());
}
