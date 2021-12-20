use proconio::input;

fn main() {
    input! {
       n:i32,k:i32,a:i32,
    }
    println!("{}", (a + k - 2) % n + 1);
}
