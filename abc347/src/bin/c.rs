use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        d:[usize;n],
    }
    let c = a + b;
    let mut e = vec![];
    for d in d {
        e.push(d % c);
    }
    e.sort();
    e.dedup();
    for i in 0..e.len() {
        if (e[i] + c - e[(i + 1) % e.len()]) % c < a {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
