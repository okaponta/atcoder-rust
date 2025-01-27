use proconio::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut s = 0;
    let mut c = 0;
    for a in a {
        if a != 0 {
            s += a;
            c += 1;
        }
    }
    println!("{}", (s + c - 1) / c);
}
