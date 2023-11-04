use std::process::exit;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[Usize1;m],
        b:[Usize1;m],
    }
    let mut used = vec![false; n];
    let mut num = vec![-1; n];
    let mut g = vec![vec![]; n];
    for i in 0..m {
        if a[i] == b[i] {
            println!("No");
            return;
        }
        g[a[i]].push(b[i]);
        g[b[i]].push(a[i]);
    }
    for i in 0..n {
        if used[i] {
            continue;
        }
        used[i] = true;
        num[i] = 0;
        dfs(i, i, &mut used, &mut num, &g);
    }
    println!("Yes");
}

fn dfs(cur: usize, prev: usize, used: &mut Vec<bool>, num: &mut Vec<i32>, g: &Vec<Vec<usize>>) {
    let own = num[cur];
    for &next in &g[cur] {
        if prev == next {
            continue;
        }
        if used[next] {
            if num[next] + own != 1 {
                println!("No");
                exit(0);
            }
        } else {
            used[next] = true;
            num[next] = 1 - own;
            dfs(next, cur, used, num, g);
        }
    }
}
