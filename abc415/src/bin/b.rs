#[rustfmt::skip]
use {itertools::*,proconio::{marker::*, *}};

fn main() {
    input! {s:Chars}
    (1..=s.len())
        .into_iter()
        .filter(|&i| s[i - 1] == '#')
        .collect_vec()
        .chunks(2)
        .for_each(|v| println!("{},{}", v[0], v[1]));
}
