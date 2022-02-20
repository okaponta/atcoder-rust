use proconio::input;

fn main() {
    input! {
        a:i32,b:i32,
    }
    let mut ans = false;
    if (a - b).abs() == 1 {
        ans = true;
    }
    if a == 10 && b == 1 || a == 1 && b == 10 {
        ans = true;
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
