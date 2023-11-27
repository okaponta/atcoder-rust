use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut row = vec![0usize; n];
    let mut col = vec![0usize; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                row[i] += 1;
                col[j] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                ans += row[i].saturating_sub(1) * col[j].saturating_sub(1);
            }
        }
    }
    println!("{}", ans);
}
