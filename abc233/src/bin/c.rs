use proconio::input;

fn main() {
    // WA
    input! {
       n:usize,x:i64,
    }
    let mut la = vec![];
    for i in 0..n {
        input! {l:usize}
        input! {
            a:[i64;l],
        }
        la[i] = a;
    }
    let mut count = 0;
    for a in &la[0] {
        count += next(0, x, *a, &la, count);
    }
    println!("{}", count);
}

fn next(index: usize, x: i64, a: i64, la: &Vec<Vec<i64>>, mut count: i32) -> i32 {
    if x % a != 0 {
        return count;
    }
    if index + 1 == la.len() {
        for ai in &la[index + 1] {
            if x == *ai {
                return count + 1;
            }
        }
    }
    for ai in &la[index + 1] {
        count += next(index + 1, x / a, *ai, la, count);
    }
    return count;
}
