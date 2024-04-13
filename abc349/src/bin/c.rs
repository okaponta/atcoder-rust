use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        mut t:Chars,
    }
    let mut i = 0;
    for c in s {
        if c == t[i].to_ascii_lowercase() {
            i += 1;
        }
        if i == 3 {
            break;
        }
    }
    if i == 3 {
        println!("Yes");
        return;
    }
    if i == 2 && t[2] == 'X' {
        println!("Yes");
        return;
    }
    println!("No");
}
