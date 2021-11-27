use proconio::input;

fn main() {
    input! {
       s:String,
    }
    let mut ans = "Yes";
    for i in 0..s.len() {
        let c: char = s.chars().nth(i).unwrap();
        if i % 2 == 0 {
            if c.is_uppercase() {
                ans = "No";
            }
        } else {
            if c.is_lowercase() {
                ans = "No";
            }
        }
    }
    println!("{}", ans);
}
