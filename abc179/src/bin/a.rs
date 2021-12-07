use proconio::input;

fn main() {
    input! {
       s:String,
    }
    println!(
        "{}{}",
        s,
        if s.chars().last().unwrap() == 's' {
            "es"
        } else {
            "s"
        }
    );
}
