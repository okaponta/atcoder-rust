use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:Chars,
    }
    println!(
        "{}",
        if (n[0] == n[1] || n[2] == n[3]) && n[1] == n[2] {
            "Yes"
        } else {
            "No"
        }
    );
}
