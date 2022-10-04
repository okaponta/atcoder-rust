use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut ans = vec![vec![0; w]; h];
    for i in 0..h {
        let mut count = 0;
        for j in 0..w {
            if s[i][j] == '.' {
                count += 1;
                ans[i][j] += count;
            } else {
                count = 0;
            }
        }
    }
    for i in 0..h {
        let mut count = 0;
        for j in (0..w).rev() {
            if s[i][j] == '.' {
                count += 1;
                ans[i][j] += count;
            } else {
                count = 0;
            }
        }
    }
    for j in 0..w {
        let mut count = 0;
        for i in 0..h {
            if s[i][j] == '.' {
                count += 1;
                ans[i][j] += count;
            } else {
                count = 0;
            }
        }
    }
    for j in 0..w {
        let mut count = 0;
        for i in (0..h).rev() {
            if s[i][j] == '.' {
                count += 1;
                ans[i][j] += count;
            } else {
                count = 0;
            }
        }
    }
    let mut a = 0;
    for i in 0..h {
        for j in 0..w {
            a = a.max(ans[i][j] - 3);
        }
    }
    println!("{}", a);
}
