use proconio::input;

fn main() {
    input! {
        a:i32,b:i32,c:i32,
    }
    println!("{}", (c - (a - b)).max(0));
}
