use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i64;n],
    }
    let c1 = calc(a.clone(), n, 0);
    let c2 = calc(a, n, 1);
    println!("{}", c1.min(c2));
}

fn calc(mut a: Vec<i64>, n: usize, o: usize) -> i64 {
    let mut c = 0;
    let mut s = 0;
    for i in 0..n {
        if i % 2 == o {
            if a[i] <= 0 {
                c += 1 - a[i];
                a[i] = 1;
            }
            s += a[i];
            if s <= 0 {
                c += 1 - s;
                s = 1;
            }
        } else {
            if 0 <= a[i] {
                c += a[i] + 1;
                a[i] = -1;
            }
            s += a[i];
            if 0 <= s {
                c += s + 1;
                s = -1;
            }
        }
    }
    c
}
