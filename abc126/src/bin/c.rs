use proconio::input;

fn main() {
    input! {
        n:usize,
        mut k:usize,
    }
    k -= 1;
    let mut prev = k.min(n);
    let mut faces = vec![n.saturating_sub(k)];
    while 0 < k {
        k /= 2;
        let s = k.min(n);
        faces.push(prev - s);
        prev = s;
    }
    let mut a = faces[0];
    let mut b = n;
    for i in 1..faces.len() {
        a <<= 1;
        b <<= 1;
        a += faces[i];
    }
    println!("{}", a as f64 / b as f64);
}
