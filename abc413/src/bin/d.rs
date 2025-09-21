use proconio::*;

fn main() {
    input! {t:usize}
    for _ in 0..t {
        case();
    }
}

fn case() {
    input! {n:usize, mut a:[i64;n]}
    if n <= 2 {
        println!("Yes");
        return;
    }
    let zero = a.iter().filter(|&i| &0 == i).count();
    if 0 < zero {
        pb(n - 1 <= zero);
        return;
    }
    let count = a.iter().map(|&i| if 0 < i { 1 } else { -1 }).sum::<i64>();
    if count.abs() == n as i64 {
        a.sort();
        pb(judge(a));
    } else if count.abs() == 1 {
        pb(judge(rearrange(count, &a)));
    } else if count.abs() == 0 {
        pb(judge(rearrange(1, &a)) || judge(rearrange(-1, &a)));
    } else {
        println!("No");
    }
}

fn rearrange(count: i64, a: &Vec<i64>) -> Vec<i64> {
    let mut pos = vec![];
    let mut neg = vec![];
    for &a in a {
        if 0 < a {
            pos.push(a);
        } else {
            neg.push(a);
        }
    }
    pos.sort();
    neg.sort();
    neg.reverse();
    let mut res = vec![];
    let mut is_pos = 0 <= count;
    while pos.len() != 0 || neg.len() != 0 {
        if is_pos {
            res.push(pos.pop().unwrap());
        } else {
            res.push(neg.pop().unwrap());
        }
        is_pos = !is_pos;
    }
    res
}

fn judge(a: Vec<i64>) -> bool {
    a.windows(3).all(|v| v[0] * v[2] == v[1] * v[1])
}

fn pb(b: bool) {
    println!("{}", if b { "Yes" } else { "No" })
}
