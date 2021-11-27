use proconio::input;

fn main() {
    input! {
       s: String,
    }
    println!("{}", s.split('.').nth(0).unwrap());
}
