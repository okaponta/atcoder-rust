use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut grid = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                grid[i][j] = true;
            }
        }
    }

    for i in 0..n {
        for j in 0..n - 5 {
            let mut count = 0;
            for k in 0..6 {
                if grid[i][j + k] {
                    count += 1;
                }
            }
            if count > 3 {
                println!("Yes");
                return;
            }
        }
    }

    for i in 0..n - 5 {
        for j in 0..n {
            let mut count = 0;
            for k in 0..6 {
                if grid[i + k][j] {
                    count += 1;
                }
            }
            if count > 3 {
                println!("Yes");
                return;
            }
        }
    }

    for i in 0..n - 5 {
        for j in 0..n - 5 {
            let mut count = 0;
            for k in 0..6 {
                if grid[i + k][j + k] {
                    count += 1;
                }
            }
            if count > 3 {
                println!("Yes");
                return;
            }
        }
    }

    for i in 0..n - 5 {
        for j in 5..n {
            let mut count = 0;
            for k in 0..6 {
                if grid[i + k][j - k] {
                    count += 1;
                }
            }
            if count > 3 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
