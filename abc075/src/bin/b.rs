use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        mut s:[Chars;h],
    }
    let mut num = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                for (di, dj) in vec![
                    (!0, 0),
                    (!0, !0),
                    (!0, 1),
                    (0, !0),
                    (0, 1),
                    (1, 0),
                    (1, !0),
                    (1, 1),
                ] {
                    let ni = i.wrapping_add(di);
                    let nj = j.wrapping_add(dj);
                    if h <= ni || w <= nj {
                        continue;
                    }
                    if s[ni][nj] == '#' {
                        num[i][j] += 1;
                    }
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!(
                "{}",
                if s[i][j] == '.' {
                    std::char::from_digit(num[i][j], 10).unwrap()
                } else {
                    s[i][j]
                }
            );
        }
        println!();
    }
}
