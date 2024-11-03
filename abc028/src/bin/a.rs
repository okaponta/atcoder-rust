#[allow(unused)]
use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
    }
    println!(
        "{}",
        if n < 60 {
            "Bad"
        } else if n < 90 {
            "Good"
        } else if n < 100 {
            "Great"
        } else {
            "Perfect"
        }
    );
}
