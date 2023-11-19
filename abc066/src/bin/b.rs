use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
    }
    loop {
        s.pop();
        if is_ok(&s) {
            println!("{}", s.len());
            return;
        }
    }
}

fn is_ok(s: &Vec<char>) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }
    for i in 0..s.len() / 2 {
        if s[i] != s[i + s.len() / 2] {
            return false;
        }
    }
    true
}
