use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        t:[i64;n],
        m:usize,
        px:[(Usize1,i64);m],
    }
    let s = t.iter().sum::<i64>();
    for (p, x) in px {
        println!("{}", s - t[p] + x);
    }
}
