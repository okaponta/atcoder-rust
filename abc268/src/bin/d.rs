use std::{collections::HashSet, process::exit};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[String;n],
        t:[String;m],
    }
    let set = t.iter().collect::<HashSet<_>>();
    let str_len = s.iter().fold(n - 1, |len, s| len + s.len());
    if 16 < str_len {
        println!("-1");
        return;
    }
    for perm in s.iter().permutations(n) {
        dfs("".to_string(), 0, &perm, 16 - str_len, &set);
    }
    println!("-1");
}

// doneの数まで処理が完了した
// freeだけアンダーバーを追加できる
fn dfs(prev: String, next: usize, perm: &Vec<&String>, free: usize, set: &HashSet<&String>) {
    let mut cur = prev + perm[next];
    if next + 1 == perm.len() {
        // 文字列完成
        if !set.contains(&cur) && 2 < cur.len() {
            println!("{}", cur);
            exit(0);
        }
    } else {
        for i in 0..=free {
            cur = cur + "_";
            dfs(cur.clone(), next + 1, perm, free - i, set);
        }
    }
}
