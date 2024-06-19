use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    let target = vec![
        vec!['d', 'r', 'e', 'a', 'm'],
        vec!['d', 'r', 'e', 'a', 'm', 'e', 'r'],
        vec!['e', 'r', 'a', 's', 'e'],
        vec!['e', 'r', 'a', 's', 'e', 'r'],
    ];
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(i) = q.pop_front() {
        if i == n {
            println!("YES");
            return;
        }
        for t in &target {
            if i + t.len() <= n && (0..t.len()).into_iter().all(|j| s[i + j] == t[j]) {
                q.push_back(i + t.len());
            }
        }
    }
    println!("NO");
}
