use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        d:usize,
    }
    let mut ans = vec![];
    let mut tmp = a;
    loop {
        ans.push(tmp);
        if tmp == b {
            break;
        }
        tmp += d;
    }
    println!("{}", ans.iter().join(" "));
}
