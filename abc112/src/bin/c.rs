use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n:usize,
        xyh:[(i64,i64,i64);n],
    }
    for (x, y) in iproduct!(0..101, 0..101) {
        let mut h = 0;
        for &(xi, yi, hi) in &xyh {
            if hi != 0 {
                h = hi + (x - xi).abs() + (y - yi).abs();
            }
        }
        if xyh
            .iter()
            .all(|&(xi, yi, hi)| (h - (x - xi).abs() - (y - yi).abs()).max(0) == hi)
        {
            println!("{} {} {}", x, y, h);
            return;
        }
    }
}
