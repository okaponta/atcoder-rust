use proconio::input;

fn main() {
    input! {
        n:usize,
        k:i64,
        xy:[(i64,i64);n],
    }
    let mut x = vec![];
    let mut y = vec![];
    for (xi, yi) in xy {
        x.push(xi);
        y.push(yi);
    }
    x.sort();
    y.sort();
    let mut lower = -1;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if count(&x, med) + count(&y, med) <= k {
            upper = med;
        } else {
            lower = med;
        }
    }
    println!("{}", upper);
}

fn count(v: &Vec<i64>, med: i64) -> i64 {
    let mut target = vec![];
    for i in 0..v.len() {
        target.push(v[i]);
    }
    for i in 0..v.len() {
        target.push(v[i] - med);
    }
    target.sort();
    let mut res = 0;
    for i in 0..v.len() {
        if v[i] < target[v.len()] {
            res += target[v.len()] - v[i];
        } else if target[v.len()] + med < v[i] {
            res += v[i] - target[v.len()] - med;
        }
    }
    res
}
