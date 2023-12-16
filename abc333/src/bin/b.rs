use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let a = diff(s[0], s[1]);
    let b = diff(t[0], t[1]);
    if a && b || !a && !b {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn diff(a: char, b: char) -> bool {
    let a = a as u8;
    let b = b as u8;
    let d = if a < b { b - a } else { a - b };
    d == 1 || d == 4
}
