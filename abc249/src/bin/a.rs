use proconio::input;

fn main() {
    input! {
        a:i32,b:i32,c:i32,
        d:i32,e:i32,f:i32,
        x:i32
    }
    let tak = distance(a, b, c, x);
    let aok = distance(d, e, f, x);
    let ans = if tak > aok {
        "Takahashi"
    } else if tak == aok {
        "Draw"
    } else {
        "Aoki"
    };
    println!("{}", ans);
}

fn distance(a: i32, b: i32, c: i32, x: i32) -> i32 {
    let mut res = (x / (a + c)) * a * b;
    if x % (a + c) > a {
        res += a * b;
    } else {
        res += (x % (a + c)) * b;
    }
    res
}
