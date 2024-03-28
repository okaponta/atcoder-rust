use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[Chars;n],
        b:[Chars;m],
    }
    for (i, j) in iproduct!(0..=n - m, 0..=n - m) {
        if is_ok(&a, &b, m, i, j) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn is_ok(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>, m: usize, i: usize, j: usize) -> bool {
    for (k, l) in iproduct!(0..m, 0..m) {
        if a[i + k][j + l] != b[k][l] {
            return false;
        }
    }
    true
}
