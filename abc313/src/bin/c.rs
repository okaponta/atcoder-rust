use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.reverse();
    let sum: usize = a.iter().sum();
    let mut ans = 0;
    for i in 0..n {
        let mut target = sum / n;
        if i < sum % n {
            target += 1;
        }
        ans += abs(target, a[i]);
    }
    println!("{}", ans / 2);
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
