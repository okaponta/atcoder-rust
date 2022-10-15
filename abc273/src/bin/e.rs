use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let mut cur = 0;
    let mut parent = vec![0; q + 1];
    let mut value = vec![0; q + 1];
    let mut note = HashMap::new();
    value[0] = -1;
    let mut ans = vec![0; q];
    for i in 0..q {
        input! {s: String}
        match s.as_str() {
            "ADD" => {
                input! {x: i32}
                parent[i + 1] = cur;
                cur = i + 1;
                value[cur] = x;
            }
            "DELETE" => cur = parent[cur],
            "SAVE" => {
                input! {y: i32}
                note.insert(y, cur);
            }
            _ => {
                input! {z: i32}
                cur = *note.get(&z).unwrap_or(&0);
            }
        }
        ans[i] = value[cur];
    }
    println!("{}", ans.iter().join(" "));
}
