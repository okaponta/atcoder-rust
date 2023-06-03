use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        sa:[(String,usize);n],
    }
    let mut min = 1 << 60;
    let mut min_idx = 0;
    for i in 0..n {
        if sa[i].1 < min {
            min = sa[i].1;
            min_idx = i;
        }
    }
    for i in min_idx..min_idx + n {
        println!("{}", sa[i % n].0);
    }
}
