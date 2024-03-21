use proconio::input;

fn main() {
    input! {
        x:usize,
    }
    println!("A{}C", if x < 1200 { "B" } else { "R" });
}
