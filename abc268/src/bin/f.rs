use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        mut s:[String;n],
    }
    s.sort_by(|a, b| score(b.to_string() + a).cmp(&score(a.to_string() + b)));
    println!("{}", score(s.iter().join("")));
}

fn score(s: String) -> usize {
    let mut x = 0;
    let mut score = 0;
    for c in s.chars() {
        if c == 'X' {
            x += 1;
        } else {
            score += c.to_digit(10).unwrap() as usize * x;
        }
    }
    score
}
