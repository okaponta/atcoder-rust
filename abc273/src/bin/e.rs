use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        q:usize,
    }
    let mut qs = vec![];
    for _ in 0..q {
        input! {
            s: Chars
        }
        if s[0] == 'D' {
            qs.push((s[0], 0));
        } else {
            input! {i:i64}
            qs.push((s[0], i));
        }
    }
    let mut cur = 0;
    let mut child = vec![0; q + 1];
    let mut parent = vec![0; q + 1];
    let mut value = vec![0; q + 1];
    let mut note = HashMap::new();
    value[0] = -1;
    let mut ans = vec![];
    for (idx, (c, i)) in qs.into_iter().enumerate() {
        if c == 'A' {
            child[cur] = idx + 1;
            parent[idx + 1] = cur;
            cur = child[cur];
            value[cur] = i;
        }
        if c == 'D' {
            cur = parent[cur];
        }
        if c == 'S' {
            note.insert(i, cur);
        }
        if c == 'L' {
            cur = *note.get(&i).unwrap_or(&0);
        }
        ans.push(value[cur]);
    }
    println!("{}", ans.iter().join(" "));
}
