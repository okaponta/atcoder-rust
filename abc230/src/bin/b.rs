use proconio::input;

fn main() {
    input! {
       s:String,
    }
    let t = "oxxoxxoxxoxx";
    println!("{}", if t.contains(&s) { "Yes" } else { "No" });
}
