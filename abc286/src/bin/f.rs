use std::io::{stdin, BufReader};

use itertools::Itertools;
use proconio::{input, source::line::LineSource};

fn main() {
    let div = vec![4, 5, 7, 9, 11, 13, 17, 19, 23];
    let mut s = vec![0];
    let d = div.len();
    for i in 0..d {
        s.push(s[i] + div[i]);
    }
    let mut a = (2..110).into_iter().collect::<Vec<_>>();
    let m = a.len();
    for i in 0..d {
        a[s[i + 1] - 1] = s[i] + 1;
    }
    println!("{}", m);
    println!("{}", a.iter().join(" "));
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        b: [usize;m],
    }
    let mut set = vec![];
    for i in 0..d {
        set.push((div[i], b[s[i]] - s[i] - 1));
    }
    println!("{}", crt(set));
}

fn crt(set: Vec<(usize, usize)>) -> usize {
    let mut res = 0;
    let mut lcm = 1;
    for (div, rem) in set {
        loop {
            if res % div == rem {
                break;
            }
            res += lcm;
        }
        lcm *= div;
    }
    res
}
