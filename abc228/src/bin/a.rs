use proconio::input;

fn main() {
    input! {
       s:i32,t:i32,x:i32,
    }
    if s > t {
        println!("{}", if s <= x || x < t { "Yes" } else { "No" });
    } else {
        println!("{}", if s <= x && x < t { "Yes" } else { "No" });
    }
}
