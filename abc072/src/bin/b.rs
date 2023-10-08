use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = vec![];
    for i in (0..s.len()).step_by(2) {
        ans.push(s[i]);
    }
    println!("{}", ans.iter().collect::<String>());
}
