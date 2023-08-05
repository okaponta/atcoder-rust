use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::io::{stdin, BufReader};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n:usize,
        k:usize
    }
    let mut ans = vec![0; n];
    let mut s = 0;
    let mut ex = vec![0; k + 1];
    for i in 0..=k {
        println!(
            "? {}",
            (0..=k)
                .into_iter()
                .filter(|j| j != &i)
                .map(|i| i + 1)
                .join(" ")
        );
        input! {
            from &mut source,
            t:u8
        }
        ex[i] = t;
        s ^= t;
    }
    for i in 0..=k {
        ans[i] = s ^ ex[i];
    }
    s ^= ans[k];
    s ^= ans[k - 1];
    for i in k + 1..n {
        println!(
            "? {}",
            (0..k - 1)
                .chain(i..i + 1)
                .into_iter()
                .map(|i| i + 1)
                .join(" ")
        );
        input! {
            from &mut source,
            t:u8
        }
        ans[i] = t ^ s;
    }
    println!("! {}", ans.iter().join(" "));
}
