use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    if n == 1 {
        println!("Hello World");
        return;
    }
    input! {a:i32,b:i32}
    println!("{}", a + b);
}
