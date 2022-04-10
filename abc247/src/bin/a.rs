use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = vec!['0'; 4];
    for i in 0..3 {
        ans[i + 1] = s[i];
    }
    println!("{}", ans.iter().collect::<String>());
}
