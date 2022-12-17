use std::io::{stdin, BufReader};

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize
    }
    let mut lr = vec![];
    let mut range = 1;
    let mut min = vec![];
    let mut max = vec![];
    while range <= n {
        min.push(lr.len());
        for j in 1..=n - range + 1 {
            lr.push((j, j + range - 1));
        }
        max.push(lr.len());
        range *= 2;
    }
    println!("{}", lr.len());
    for (l, r) in lr {
        println!("{} {}", l, r);
    }
    input! {
        from &mut source,
        q: usize
    }
    for _ in 0..q {
        input! {
            from &mut source,
            l: usize,
            r: usize,
        }
        let len = r - l + 1;
        let mut temp = 1;
        let mut pow = 0;
        while temp <= len {
            pow += 1;
            temp *= 2;
        }
        pow -= 1;
        println!("{} {}", min[pow] + l, max[pow] + r - n);
    }
}
