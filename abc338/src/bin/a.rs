use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    if s[0].is_ascii_uppercase() {
        if (1..s.len()).all(|i| s[i].is_ascii_lowercase()) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
