use proconio::{input, marker::Chars};

fn main() {
    input! {
       h:usize,w:usize,
       c:[Chars;h],
    }
    let mut grid = vec![vec![0; w]; h];
    grid[0][0] = 1;
    for j in 1..w {
        if c[0][j] == '#' {
            continue;
        }
        if grid[0][j - 1] != 0 {
            grid[0][j] = grid[0][j - 1] + 1;
        }
    }
    for i in 1..h {
        for j in 0..w {
            if c[i][j] == '#' {
                continue;
            }
            if j == 0 {
                if grid[i - 1][j] != 0 {
                    grid[i][0] = grid[i - 1][0] + 1;
                }
                continue;
            }
            let mut cand1 = 0;
            let mut cand2 = 0;
            if grid[i - 1][j] != 0 {
                cand1 = grid[i - 1][j] + 1;
            }
            if grid[i][j - 1] != 0 {
                cand2 = grid[i][j - 1] + 1;
            }
            grid[i][j] = cand1.max(cand2);
        }
    }
    let ans = grid
        .iter()
        .map(|e| e.iter().max().unwrap_or(&0))
        .max()
        .unwrap();
    println!("{}", ans);
}
