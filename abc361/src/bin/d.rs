use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut s:Chars,
        mut t:Chars,
    }
    if s == t {
        println!("0");
        return;
    }
    s.push('.');
    s.push('.');
    t.push('.');
    t.push('.');
    let mut q = VecDeque::new();
    q.push_back((s.clone(), 0));
    let mut set = HashSet::new();
    set.insert(s.clone());
    while let Some((s, i)) = q.pop_front() {
        let dot = s.iter().position(|&c| c == '.').unwrap();
        for j in 0..=n {
            if j == dot || j == dot + 1 || j + 1 == dot {
                continue;
            }
            let mut ss = s.clone();
            ss.swap(j, dot);
            ss.swap(j + 1, dot + 1);
            if set.contains(&ss) {
                continue;
            }
            if ss == t {
                println!("{}", i + 1);
                return;
            }
            set.insert(ss.clone());
            q.push_back((ss, i + 1));
        }
    }
    println!("{}", -1);
}
