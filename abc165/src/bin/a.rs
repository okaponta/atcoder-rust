use proconio::input;

fn main() {
    input! {
       k:i32,a:i32,b:i32,
    }
    for i in a..=b {
        if i % k == 0 {
            println!("OK");
            return;
        }
    }
    println!("NG");
}
