use proconio::{fastout, input};

const MAX: i64 = 1_000_000_000;

#[fastout]
fn main() {
    input! {
       n:usize,
       at:[(i64,u8);n],
       q:usize,
       mut x:[i64;q],
    }
    let mut min = -MAX;
    let mut max = MAX;
    let mut tmp = 0;
    for (a, t) in at {
        if t == 1 {
            tmp += a;
            min += a;
            max += a;
        } else if t == 2 {
            if min < a {
                min = a;
                max = max.max(min);
            }
        } else {
            if a < max {
                max = a;
                min = min.min(max);
            }
        }
    }
    for i in 0..q {
        x[i] += tmp;
        if x[i] < min {
            x[i] = min;
        }
        if max < x[i] {
            x[i] = max;
        }
        println!("{}", x[i]);
    }
}
