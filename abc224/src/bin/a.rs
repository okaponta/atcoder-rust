use proconio::input;

fn main() {
    input! {
       s:String,
    }
    println!(
        "{}",
        if s.chars().last().unwrap() == 'r' {
            "er"
        } else {
            "ist"
        }
    );
}
