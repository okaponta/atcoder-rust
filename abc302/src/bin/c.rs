use std::process::exit;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
    }
    let mut used = vec![false; n];
    dfs(&mut used, 0, 0, n, m, &s);
    println!("No");
}

fn dfs(used: &mut Vec<bool>, prev: usize, depth: usize, n: usize, m: usize, s: &Vec<Vec<char>>) {
    if depth == n {
        println!("Yes");
        exit(0);
    }
    for i in 0..n {
        if used[i] {
            continue;
        }
        if depth == 0 {
            used[i] = true;
            dfs(used, i, 1, n, m, s);
            used[i] = false;
        } else {
            let mut count = 0;
            for j in 0..m {
                if s[prev][j] != s[i][j] {
                    count += 1;
                }
            }
            if count == 1 {
                used[i] = true;
                dfs(used, i, depth + 1, n, m, s);
                used[i] = false;
            }
        }
    }
}
