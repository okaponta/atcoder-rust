use std::io::{stdin, BufReader};

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize
    }
    let mut lower = 0;
    let mut upper = n - 1;
    while upper - lower != 1 {
        let mid = (lower + upper) / 2;
        println!("? {}", mid + 1);
        input! {
            from &mut source,
            a: u8
        }
        if a == 0 {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    println!("! {}", upper);
}
