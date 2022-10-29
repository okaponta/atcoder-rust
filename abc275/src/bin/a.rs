use proconio::input;

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let mut s = vec![];
    for i in 0..n {
        s.push((h[i], i));
    }
    s.sort();
    s.reverse();
    println!("{}", s[0].1 + 1);
}
