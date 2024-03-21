use proconio::input;

fn main() {
    input! {
        a:i128,
        b:i128,
    }
    let l = lcm(a, b);
    if 10i128.pow(18u32) < l {
        println!("Large");
    } else {
        println!("{}", l);
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

// 最小公倍数
// 計算量はO(log min(a,b))
fn lcm(a: i128, b: i128) -> i128 {
    a * b / gcd(a, b)
}
