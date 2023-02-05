use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut ac:[(usize,usize);m],
    }
    ac.sort_by_key(|k| k.1);
    let mut g = n;
    for i in 0..m {
        g = gcd(g, ac[i].0);
    }
    if n % g == 0 && g != 1 {
        println!("-1");
        return;
    }
    let mut ans = 0;
    g = n;
    for (a, c) in ac {
        let after = gcd(g, a);
        if g != after {
            ans += (g - after) * c;
        }
        g = after;
        if g == 1 {
            break;
        }
    }
    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
