use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
        q:usize,
    }
    for _ in 0..q {
        input! {q: u8, k:Usize1}
        if q == 1 {
            input! {x:usize}
            a[k] = x;
        } else {
            println!("{}", a[k]);
        }
    }
}
