use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        x:i64,y:i64,
        abc:[i64;3],
    }
    for p in abc.iter().permutations(3) {
        for (i, j, k) in iproduct!(0..2, 0..2, 0..2) {
            let mut xy = (x, y);
            xy = sub(i, *p[0], xy.0, xy.1);
            if xy.0 <= 0 || xy.1 <= 0 {
                continue;
            }
            xy = sub(j, *p[1], xy.0, xy.1);
            if xy.0 <= 0 || xy.1 <= 0 {
                continue;
            }
            xy = sub(k, *p[2], xy.0, xy.1);
            if xy.0 < 0 || xy.1 < 0 {
                continue;
            }
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn sub(i: i32, a: i64, x: i64, y: i64) -> (i64, i64) {
    if i == 0 {
        return (x - cel(a, y), y);
    }
    return (x, y - cel(a, x));
}

fn cel(a: i64, x: i64) -> i64 {
    (a + x - 1) / x
}
