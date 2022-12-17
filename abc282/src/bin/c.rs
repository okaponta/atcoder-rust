use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut s:Chars
    }
    let mut inner = false;
    for i in 0..n {
        if s[i] == '"' {
            inner = !inner;
            continue;
        }
        if !inner && s[i] == ',' {
            s[i] = '.';
        }
    }
    println!("{}", s.into_iter().collect::<String>());
}
