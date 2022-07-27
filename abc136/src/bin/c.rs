use proconio::input;

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let mut max = 1_000_000_001;
    for i in (0..n).rev() {
        if max + 1 < h[i] {
            println!("No");
            return;
        }
        max = max.min(h[i]);
    }
    println!("Yes");
}
