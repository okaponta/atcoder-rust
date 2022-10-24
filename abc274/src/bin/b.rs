use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        c:[Chars;h],
    }
    let mut ans = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                ans[j] += 1;
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}
