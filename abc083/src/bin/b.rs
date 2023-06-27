use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
    }
    let mut ans = 0;
    for i in 1..=n {
        let s = sum_digits(i);
        if a <= s && s <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}

fn sum_digits(mut n: usize) -> usize {
    let mut res = 0;
    while 0 < n {
        res += n % 10;
        n /= 10;
    }
    res
}
