use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        m:[usize;n],
    }
    let mut s = 0;
    let mut min = 1 << 60;
    for i in 0..n {
        s += m[i];
        min = min.min(m[i]);
    }
    println!("{}", (x - s) / min + n);
}
