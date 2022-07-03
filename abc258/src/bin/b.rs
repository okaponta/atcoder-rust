use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        a:[Chars;n],
    }
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = a[i][j].to_digit(10).unwrap();
        }
    }
    let dx = vec![0, 1, 1, 1, 0, -1, -1, -1];
    let dy = vec![1, 1, 0, -1, -1, -1, 0, 1];
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..8 {
                let mut cand = String::new();
                cand.push(a[i][j]);
                let mut x = i;
                let mut y = j;
                for _ in 1..n {
                    let nextx = x as i32 + dx[k];
                    let nexty = y as i32 + dy[k];
                    x = nextx.rem_euclid(n as i32) as usize;
                    y = nexty.rem_euclid(n as i32) as usize;
                    cand.push(a[x][y]);
                }
                ans = ans.max(cand.parse::<i64>().unwrap());
            }
        }
    }
    println!("{}", ans);
}
