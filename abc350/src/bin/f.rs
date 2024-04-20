use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    let mut rev = vec![0; n];
    let mut g = vec![0; n];
    let mut tmp = vec![];
    let mut count = 0;
    for i in 0..n {
        if s[i] == '(' {
            tmp.push(i);
            count += 1;
        }
        if s[i] == ')' {
            let j = tmp.pop().unwrap();
            g[i] = j;
            g[j] = i;
            count -= 1;
        }
        rev[i] = count;
    }
    let mut ans = vec![];
    let mut q = VecDeque::new();
    q.push_front((0, true));
    while let Some((i, dir)) = q.pop_front() {
        if i == n {
            break;
        }
        let tmp = i;
        if dir {
            // 正方向
            if s[tmp] == ')' {
                // おわり
            } else if s[tmp] == '(' {
                // 次
                q.push_front((g[tmp] + 1, true));
                q.push_front((g[tmp] - 1, false));
            } else {
                ans.push(cv(s[tmp], rev[tmp]));
                q.push_front((tmp + 1, true));
            }
        } else {
            // 逆方向
            if s[tmp] == '(' {
                // おわり
            } else if s[tmp] == ')' {
                // 次
                q.push_front((g[tmp] - 1, false));
                q.push_front((g[tmp] + 1, true));
            } else {
                ans.push(cv(s[tmp], rev[tmp]));
                q.push_front((tmp - 1, false));
            }
        }
    }
    println!("{}", ans.iter().join(""));
}

fn cv(c: char, count: usize) -> char {
    if count % 2 == 0 {
        return c;
    }
    if c.is_ascii_lowercase() {
        return c.to_ascii_uppercase();
    } else {
        return c.to_ascii_lowercase();
    }
}
