#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};
use num_integer::Roots;

fn main() {
    input! {
        n:usize,
    }
    for a in 1.. {
        if n < a * a * a {
            break;
        }
        if n % a != 0 {
            continue;
        }
        let b1 = (n * 4) / a;
        if b1 < a * a {
            continue;
        }
        let b2 = b1 - a * a;
        if b2 % 3 != 0 {
            continue;
        }
        let b3 = b2 / 3;
        let b4 = b3.sqrt();
        if b4 * b4 != b3 {
            continue;
        }
        if (a + b4) % 2 != 0 || (b4 - a) % 2 != 0 || (a + b4) == 0 || (b4 - a) == 0 {
            continue;
        }
        println!("{} {}", (a + b4) / 2, (b4 - a) / 2);
        return;
    }
    println!("-1");
}
