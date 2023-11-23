use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut s:Chars,
        t:Chars,
    }
    let mut q = VecDeque::new();
    for i in 0..=n - m {
        ope(i, m, &mut s, &t, &mut q);
    }
    while let Some(i) = q.pop_front() {
        for j in i.saturating_sub(m)..(i + m).min(n - m) {
            if (0..m).into_iter().all(|k| s[j + k] == '#') {
                continue;
            }
            ope(j, m, &mut s, &t, &mut q);
        }
    }
    let ok = (0..n).into_iter().all(|i| s[i] == '#');
    println!("{}", if ok { "Yes" } else { "No" });
}

fn ope(i: usize, m: usize, s: &mut Vec<char>, t: &Vec<char>, q: &mut VecDeque<usize>) {
    if is_ok(i, m, &s, &t) {
        q.push_back(i);
        for j in 0..m {
            s[i + j] = '#';
        }
    }
}

fn is_ok(i: usize, m: usize, s: &Vec<char>, t: &Vec<char>) -> bool {
    for j in 0..m {
        if s[i + j] != '#' && s[i + j] != t[j] {
            return false;
        }
    }
    true
}
