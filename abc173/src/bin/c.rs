use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
       h:usize,w:usize,k:usize,
       c:[Chars;h]
    }
    let count = iproduct!(0..1 << h, 0..1 << w)
        .filter(|&(row, col)| {
            iproduct!(
                (0..h).filter(|&i| (row >> i) % 2 == 1),
                (0..w).filter(|&j| (col >> j) % 2 == 1)
            )
            .filter(|&(i, j)| c[i][j] == '#')
            .count()
                == k
        })
        .count();
    println!("{}", count);
}
