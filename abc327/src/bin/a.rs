use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    for i in 0..n - 1 {
        if (s[i] == 'b' && s[i + 1] == 'a') || (s[i] == 'a' && s[i + 1] == 'b') {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
