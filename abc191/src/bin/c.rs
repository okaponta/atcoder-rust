use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize, w:usize,
        s:[Chars;h],
    }
    let mut ans = 0;
    for i in 1..h {
        for j in 1..w {
            let mut count = 0;
            if s[i][j] == '#' {
                count += 1;
            }
            if s[i - 1][j] == '#' {
                count += 1;
            }
            if s[i][j - 1] == '#' {
                count += 1;
            }
            if s[i - 1][j - 1] == '#' {
                count += 1;
            }
            if count == 1 || count == 3 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
