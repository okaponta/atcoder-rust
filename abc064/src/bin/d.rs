use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        mut s:Chars,
    }
    for _ in 0..count_bracket(&s, '(') {
        s.push(')');
    }
    s.reverse();
    for _ in 0..count_bracket(&s, ')') {
        s.push('(');
    }
    s.reverse();
    println!("{}", s.iter().join(""));
}

fn count_bracket(s: &Vec<char>, target: char) -> i32 {
    let mut i = 0;
    let mut count = 0;
    while i < s.len() {
        let b = if s[i] == target { 1 } else { -1 };
        count = (count + b).max(0);
        i += 1;
    }
    count
}
