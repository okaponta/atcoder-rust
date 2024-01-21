use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        k:usize,
        s:[Chars;h],
    }
    let mut ans = h * w + 1;
    for i in 0..h {
        let mut count = 0;
        let mut point = 0;
        for j in 0..w {
            if s[i][j] == 'o' {
                count += 1;
            } else if s[i][j] == '.' {
                count += 1;
                point += 1;
            } else {
                count = 0;
                point = 0;
            }
            if count == k {
                ans = ans.min(point);
                count -= 1;
                if s[i][j + 1 - k] == '.' {
                    point -= 1;
                }
            }
        }
    }
    for j in 0..w {
        let mut count = 0;
        let mut point = 0;
        for i in 0..h {
            if s[i][j] == 'o' {
                count += 1;
            } else if s[i][j] == '.' {
                count += 1;
                point += 1;
            } else {
                count = 0;
                point = 0;
            }
            if count == k {
                ans = ans.min(point);
                count -= 1;
                if s[i + 1 - k][j] == '.' {
                    point -= 1;
                }
            }
        }
    }
    if ans == h * w + 1 {
        println!("-1");
        return;
    }
    println!("{}", ans);
}
