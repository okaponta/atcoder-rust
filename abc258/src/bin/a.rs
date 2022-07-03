use proconio::input;

fn main() {
    input! {
        k:usize,
    }
    println!("{}:{:<02}", 21 + k / 60, k % 60);
}
