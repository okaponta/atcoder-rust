use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut ac:[(usize,usize);m],
    }
    ac.sort_by_key(|k| k.1);
    let mut g = n;
    let mut ans = 0;
    for (a, c) in ac {
        let after = gcd(g, a);
        if g != after {
            ans += (g - after) * c;
        }
        g = after;
        if g == 1 {
            println!("{}", ans);
            return;
        }
    }
    println!("-1");
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
