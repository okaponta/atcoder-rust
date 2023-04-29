use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        c:[Chars;h],
    }
    let mut ans = vec![0; h.min(w)];
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if c[i][j] == '#'
                && c[i + 1][j + 1] == '#'
                && c[i + 1][j - 1] == '#'
                && c[i - 1][j + 1] == '#'
                && c[i - 1][j - 1] == '#'
            {
                // 中心
                let mut size = 0;
                while i + size + 1 < h && j + size + 1 < w && c[i + size + 1][j + size + 1] == '#' {
                    size += 1;
                }
                ans[size - 1] += 1;
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}
