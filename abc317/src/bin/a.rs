use proconio::input;

fn main() {
    input! {
        n:usize,
        h:usize,
        x:usize,
        p:[usize;n],
    }
    for i in 0..n {
        if x - h <= p[i] {
            println!("{}", i + 1);
            return;
        }
    }
}
