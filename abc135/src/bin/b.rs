use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        p:[Usize1;n],
    }
    let mut count = 0;
    for i in 0..n {
        if i != p[i] {
            count += 1;
        }
    }
    println!("{}", if count <= 2 { "YES" } else { "NO" });
}
