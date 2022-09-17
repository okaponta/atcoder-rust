use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:[Chars;10],
    }
    let mut ans = vec![0; 4];
    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                ans[1] = i + 1;
                ans[2] = j + 1;
                break;
            }
        }
    }
    for i in (0..10).rev() {
        for j in (0..10).rev() {
            if s[i][j] == '#' {
                ans[0] = i + 1;
                ans[3] = j + 1;
                break;
            }
        }
    }
    println!("{} {}", ans[0], ans[1]);
    println!("{} {}", ans[2], ans[3]);
}
