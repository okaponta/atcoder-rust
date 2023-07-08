use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut s = 0;
    let mut m = n;
    while 0 < m {
        s += m % 10;
        m /= 10;
    }
    println!("{}", if n % s == 0 { "Yes" } else { "No" });
}
