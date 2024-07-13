use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        lr:[(i64,i64);n],
    }
    let mut min = 0;
    let mut max = 0;
    for i in 0..n {
        min += lr[i].0;
        max += lr[i].1;
    }
    if 0 < min || max < 0 {
        println!("No");
        return;
    }
    println!("Yes");
    let mut ans = vec![];
    let mut rem = -min;
    for i in 0..n {
        if rem == 0 {
            ans.push(lr[i].0);
            continue;
        }
        let mut tmp = lr[i].1;
        rem -= lr[i].1 - lr[i].0;
        if rem < 0 {
            tmp += rem;
            rem = 0;
        }
        ans.push(tmp);
    }
    println!("{}", ans.iter().join(" "));
}
