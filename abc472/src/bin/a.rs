#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::*};

fn main() {
    input! {
        mut s:Chars,
    }
    println!(
        "{}",
        s.into_iter()
            .map(|c| if c != 'A' { '.' } else { c })
            .collect::<String>()
    );
}
