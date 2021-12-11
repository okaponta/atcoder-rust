use proconio::input;

fn main() {
    input! {
       a:i32,
    }
    println!("{}", a + a * a + a * a * a);
}
