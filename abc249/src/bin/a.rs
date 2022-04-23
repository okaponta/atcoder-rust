use proconio::input;

fn main() {
    input! {
        a:i32,b:i32,c:i32,
        d:i32,e:i32,f:i32,
        x:i32
    }
    let mut t = (x / (a + c)) * a * b;
    let mut ao = (x / (d + f)) * d * e;
    let xt = x % (a + c);
    let xa = x % (d + f);
    if xt > a {
        t += a * b;
    } else {
        t += xt * b;
    }
    if xa > d {
        ao += d * e;
    } else {
        ao += xa * e;
    }
    let ans = if t > ao {
        "Takahashi"
    } else if t == ao {
        "Draw"
    } else {
        "Aoki"
    };
    println!("{}", ans);
}
