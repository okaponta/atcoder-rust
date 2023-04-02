use std::mem::swap;

use proconio::input;

fn main() {
    input! {
        mut a:usize,
        mut b:usize,
        c:usize,
        mut x:usize,
        mut y:usize,
    }
    if y < x {
        swap(&mut a, &mut b);
        swap(&mut x, &mut y);
    }
    println!(
        "{}",
        (x * a + y * b).min(c * 2 * x + (y - x) * b).min(c * 2 * y)
    );
}
