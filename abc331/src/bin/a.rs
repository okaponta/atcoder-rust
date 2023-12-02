use proconio::input;

fn main() {
    input! {
        mm:usize,
        dd:usize,
        mut y:usize,
        mut m:usize,
        mut d:usize,
    }
    d += 1;
    if dd < d {
        m += 1;
        d = 1;
    }
    if mm < m {
        y += 1;
        m = 1;
    }
    println!("{} {} {}", y, m, d);
}
