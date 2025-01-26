use proconio::*;

fn main() {
    input! {
        n:usize,
        k:usize,
        t:[[usize;k];n],
    }
    dfs(0, 0, &t, n, k);
    println!("Nothing");
}

fn dfs(i: usize, tmp: usize, t: &Vec<Vec<usize>>, n: usize, k: usize) {
    if i == n {
        if tmp == 0 {
            println!("Found");
            std::process::exit(0);
        }
        return;
    }
    for j in 0..k {
        dfs(i + 1, tmp ^ t[i][j], t, n, k);
    }
}
