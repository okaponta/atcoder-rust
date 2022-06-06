use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        k:i64,
        q:usize,
        a:[Usize1;q],
    }
    let mut points = vec![k - q as i64; n];
    for ai in a {
        points[ai] += 1;
    }
    for p in points {
        println!("{}", if 0 < p { "Yes" } else { "No" });
    }
}
