use proconio::input;

fn main() {
    input! {
        n:usize,
        m:i64,
        mut a:[i64;n],
    }
    let mut lower = -1;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if !is_ok(med, m, n, &a) {
            lower = med;
        } else {
            upper = med;
        }
    }
    let mut count = 0;
    for i in 0..n {
        while upper < a[i] {
            a[i] >>= 1;
            count += 1;
        }
    }
    a.sort();
    a.reverse();
    let mut sum = 0;
    for i in 0..n {
        if i < (m - count) as usize {
            a[i] >>= 1;
        }
        sum += a[i];
    }
    println!("{}", sum);
}

fn is_ok(med: i64, m: i64, n: usize, a: &Vec<i64>) -> bool {
    let mut count = 0;
    for i in 0..n {
        let mut j = 0;
        while med < a[i] >> j {
            j += 1;
        }
        count += j;
    }
    count <= m
}
