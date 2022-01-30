use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
    }
    let mut start = 0;
    let mut end = s.len();
    while s[start] == 'a' {
        start += 1;
        if start == s.len() {
            println!("Yes");
            return;
        }
    }
    while s[end - 1] == 'a' {
        end -= 1;
    }
    let target = &s[start..end];
    for i in 0..target.len() {
        if target[i] != target[target.len() - 1 - i] {
            println!("No");
            return;
        }
    }
    if start > s.len() - end {
        println!("No");
        return;
    }
    println!("Yes");
}
