use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!("{:<02}", n % 100);
}
