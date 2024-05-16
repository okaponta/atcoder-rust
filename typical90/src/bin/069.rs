use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    if n == 1 {
        println!("{k}");
        return;
    }
    if n == 2 && k < 3 {
        println!("{}", k * (k - 1));
        return;
    }
    let modulo = 1_000_000_007;
    let ans = (((k * (k - 1)) % modulo) * pow(k - 2, n - 2, modulo)) % modulo;
    println!("{}", ans);
}

fn pow(mut x: usize, mut n: usize, modulo: usize) -> usize {
    x %= modulo;
    let mut ret = if x == 0 { 0 } else { 1 };
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % modulo;
        }
        x = x * x % modulo;
        n >>= 1;
    }
    ret
}
