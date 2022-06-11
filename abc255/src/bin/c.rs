use proconio::input;

fn main() {
    input! {
        x:i64,
        a:i64,
        d:i64,
        n:i64,
    }
    if d == 0 {
        println!("{}", (x - a).abs());
        return;
    }
    let end = a + (n - 1) * d;
    let min = a.min(end);
    let max = a.max(end);
    let c = d.abs();
    if x < min {
        println!("{}", min - x);
        return;
    }
    if max < x {
        println!("{}", x - max);
        return;
    }
    let t = x - min;
    let a1 = t % c;
    let a2 = c - a1;
    println!("{}", a1.min(a2));
}
