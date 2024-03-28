use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut a = 0;
    let mut z = 0;
    for i in 0..s.len() {
        if s[i] == 'Z' {
            z = i;
        }
    }
    for i in (0..s.len()).rev() {
        if s[i] == 'A' {
            a = i;
        }
    }
    println!("{}", z - a + 1);
}
