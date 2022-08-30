use proconio::input;

fn main() {
    input! {
        w:usize,
        h:usize,
        x:usize,
        y:usize,
    }
    let ans = (w * h) as f64 / 2.0;
    if x * 2 == w && y * 2 == h {
        println!("{} {}", ans, 1);
    } else {
        println!("{} {}", ans, 0);
    }
}
