use std::io::{stdin, BufReader};

use itertools::Itertools;
use proconio::{input, source::line::LineSource};

fn main() {
    let kitei = vec![4, 5, 7, 9, 11, 13, 17, 19, 23];
    //let kitei = vec![5, 7];
    let mut a = vec![];
    let mut tmp = 1;
    for i in kitei.clone() {
        for j in 0..i {
            if j == i - 1 {
                a.push(tmp);
                continue;
            }
            a.push(tmp + j + 1);
        }
        tmp += i;
    }
    let m = a.len();
    println!("{}", m);
    println!("{}", a.iter().join(" "));
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        b: [usize;m],
    }
    // 5でわったあまり
    let mut ans = b[0] - 1;
    let mut lcm = 4;
    let mut kijun = 4;
    for i in kitei {
        if i == 4 {
            continue;
        }
        let rem = b[kijun] - 1 - kijun;
        loop {
            if ans % i == rem {
                break;
            }
            ans += lcm;
        }
        lcm *= i;
        kijun += i;
    }
    println!("{}", ans);
}
