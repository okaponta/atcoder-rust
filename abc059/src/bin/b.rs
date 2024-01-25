use proconio::{input, marker::Chars};

fn main() {
    input! {
        a:Chars,
        b:Chars,
    }
    if a.len() < b.len() {
        println!("LESS");
    } else if a.len() == b.len() {
        for i in 0..a.len() {
            if a[i] < b[i] {
                println!("LESS");
                return;
            } else if a[i] > b[i] {
                println!("GREATER");
                return;
            }
        }
        println!("EQUAL");
    } else {
        println!("GREATER");
    }
}
