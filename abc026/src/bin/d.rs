use std::f64::consts::PI;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:f64, b:f64, c:f64,
    }
    let mut lower = 0.0;
    let mut upper = 200.0;
    while upper - lower > 1e-12 {
        let med = (lower + upper) / 2.0;
        if f(med, a, b, c) < 100.0 {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", upper);
}

fn f(t: f64, a: f64, b: f64, c: f64) -> f64 {
    a * t + b * (c * t * PI).sin()
}
