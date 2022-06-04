use proconio::input;

fn main() {
    input! {
        s:String,
    }
    println!(
        "{}",
        if s == "Sunny" {
            "Cloudy"
        } else if s == "Cloudy" {
            "Rainy"
        } else {
            "Sunny"
        }
    );
}
