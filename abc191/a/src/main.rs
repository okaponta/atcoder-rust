use proconio::input;

fn main() {
    input! {
       v:i32,t:i32,s:i32,d:i32,
    }
    println!(
        "{}",
        if v * t <= d && d <= v * s {
            "No"
        } else {
            "Yes"
        }
    );
}
