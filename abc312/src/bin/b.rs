use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
    }
    let mut black = vec![];
    for (i, j) in iproduct!(0..3, 0..3) {
        black.push((i, j));
        black.push((6 + i, 6 + j));
    }
    let mut white = vec![];
    for i in 0..4 {
        white.push((3, i));
        white.push((i, 3));
        white.push((5, 8 - i));
        white.push((8 - i, 5));
    }
    let mut ans = vec![];
    for (i, j) in iproduct!(0..n - 8, 0..m - 8) {
        if black.iter().all(|(k, l)| s[i + k][j + l] == '#')
            && white.iter().all(|(k, l)| s[i + k][j + l] == '.')
        {
            ans.push((i + 1, j + 1));
        }
    }
    for (i, j) in ans {
        println!("{} {}", i, j);
    }
}
