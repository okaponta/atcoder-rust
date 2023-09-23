use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:Chars,
    }
    for i in 1..n.len() {
        if n[i - 1] <= n[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
