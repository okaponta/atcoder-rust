use proconio::input;

fn main() {
    input! {
       a:i32,
       b:i32,
    }
    let mut ans = 4;
    if a + b >= 15 && b >= 8 {
        ans = 1;
    } else if a + b >= 10 && b >= 3 {
        ans = 2;
    } else if a + b >= 3 {
        ans = 3;
    }
    println!("{}", ans);
}
