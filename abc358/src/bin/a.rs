use proconio::input;

fn main() {
    input! {
        s:String,
        t:String,
    }
    println!(
        "{}",
        if &s == "AtCoder" && &t == "Land" {
            "Yes"
        } else {
            "No"
        }
    );
}
