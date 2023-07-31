use proconio::input;

fn main() {
    input! {
        s:String,
    }
    let str = "ACEGBDFAC".to_string();
    println!("{}", if str.contains(&s) { "Yes" } else { "No" });
}
