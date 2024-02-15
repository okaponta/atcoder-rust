use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(i64,i64);n],
        cd:[(i64,i64);m],
    }
    for (a, b) in ab {
        let mut ans = 0;
        let mut min = 1 << 60;
        for i in 0..m {
            let d = (a - cd[i].0).abs() + (b - cd[i].1).abs();
            if d < min {
                min = d;
                ans = i + 1;
            }
        }
        println!("{}", ans);
    }
}
