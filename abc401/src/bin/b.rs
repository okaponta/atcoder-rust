#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let mut count = 0;
    let mut login = false;
    for s in s {
        if s == "login" {
            login = true;
        } else if s == "logout" {
            login = false;
        } else if s == "private" {
            if !login {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
