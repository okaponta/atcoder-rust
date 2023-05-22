use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] != 's' {
            continue;
        }
        'a: for (k, l) in iproduct!(vec![!0, 0, 1], vec![!0, 0, 1]) {
            let mut tmpi = i;
            let mut tmpj = j;
            let mut ans = vec![(i, j)];
            for c in vec!['n', 'u', 'k', 'e'] {
                tmpi = tmpi.wrapping_add(k);
                tmpj = tmpj.wrapping_add(l);
                if h <= tmpi || w <= tmpj || s[tmpi][tmpj] != c {
                    continue 'a;
                }
                ans.push((tmpi, tmpj));
            }
            for (i, j) in ans {
                println!("{} {}", i + 1, j + 1);
            }
            return;
        }
    }
}
