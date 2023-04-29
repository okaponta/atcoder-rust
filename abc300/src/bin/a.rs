use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        c:[usize;n],
    }
    let s = a + b;
    for i in 0..n {
        if s == c[i] {
            println!("{}", i + 1);
            return;
        }
    }
}
