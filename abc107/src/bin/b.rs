use std::collections::HashSet;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        a:[Chars;h],
    }
    let mut skip_row = HashSet::new();
    for i in 0..h {
        if a[i].iter().all(|c| c == &'.') {
            skip_row.insert(i);
        }
    }
    let mut skip_col = HashSet::new();
    for j in 0..w {
        if a.iter().all(|v| v[j] == '.') {
            skip_col.insert(j);
        }
    }
    for i in 0..h {
        if skip_row.contains(&i) {
            continue;
        }
        for j in 0..w {
            if skip_col.contains(&j) {
                continue;
            }
            print!("{}", a[i][j]);
        }
        println!();
    }
}
