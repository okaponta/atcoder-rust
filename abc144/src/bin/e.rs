use proconio::input;

fn main() {
    input! {
        n:usize,
        k:i64,
        mut a:[i64;n],
        mut f:[i64;n],
    }
    a.sort();
    f.sort();
    f.reverse();
    let mut lower = -1;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if !is_ok(med, k, n, &a, &f) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", upper);
}

fn is_ok(med: i64, k: i64, n: usize, a: &Vec<i64>, f: &Vec<i64>) -> bool {
    let mut count = 0;
    for i in 0..n {
        count += (a[i] - med / f[i]).max(0);
    }
    count <= k
}
