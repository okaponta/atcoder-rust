use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    let mut a = vec![];
    let mut b = vec![];
    for (x, y) in xy {
        if (x + y) % 2 == 0 {
            a.push(((x + y) / 2, (x - y) / 2));
        } else {
            b.push(((x - 1 + y) / 2, (x - 1 - y) / 2));
        }
    }
    let ans = calc(a) + calc(b);
    println!("{}", ans);
}

fn calc(v: Vec<(i64, i64)>) -> i64 {
    if v.len() == 0 {
        return 0;
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..v.len() {
        a.push(v[i].0);
        b.push(v[i].1);
    }
    calc2(a) + calc2(b)
}

fn calc2(mut v: Vec<i64>) -> i64 {
    v.sort();
    let mut res = 0;
    let mut tmp = v[0];
    for i in 1..v.len() {
        res += i as i64 * v[i] - tmp;
        tmp += v[i];
    }
    res
}
