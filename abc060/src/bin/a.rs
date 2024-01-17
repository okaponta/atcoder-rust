use proconio::{input, marker::Chars};

fn main() {
    input! {
        a:Chars,
        b:Chars,
        c:Chars,
    }
    let cond = a[a.len() - 1] == b[0] && b[b.len() - 1] == c[0];
    println!("{}", if cond { "YES" } else { "NO" });
}
