use proconio::input;

fn main() {
    input! {
        n:usize,
        mut p:usize,
        q:usize,
        d:[usize;n],
    }
    for d in d {
        p = p.min(q + d);
    }
    println!("{}", p);
}
