use proconio::{input, marker::Chars};

fn main() {
    input! {
        x:Chars,
        m:i64,
    }
    if x.len() == 1 {
        let digit = x[0].to_digit(10).unwrap() as i64;
        println!("{}", if digit > m { "0" } else { "1" });
        return;
    }
    let min = x.iter().map(|c| c.to_digit(10).unwrap()).max().unwrap() as i64;
    let mut lower = min;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(&x, med, m) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower - min);
}

fn is_ok(x: &Vec<char>, med: i64, m: i64) -> bool {
    let mut sum: i64 = 0;
    let mut place = 1;
    for c in x.iter().rev() {
        let digit = c.to_digit(10).unwrap() as i64;
        sum = sum.saturating_add(digit.saturating_mul(place));
        place = place.saturating_mul(med);
    }
    sum <= m
}
