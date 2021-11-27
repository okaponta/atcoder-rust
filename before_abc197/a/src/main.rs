use proconio::input;

fn main() {
    input! {
       s: String,
    }
    let c: Vec<char> = s.chars().collect();
    println!("{}{}{}", c[1], c[2], c[0]);
}
