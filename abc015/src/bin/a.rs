use proconio::*;

fn main() {
    input! {
        a:String,
        b:String,
    }
    println!("{}", if a.len() > b.len() { a } else { b });
}
