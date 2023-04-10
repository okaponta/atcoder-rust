use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let i = *a.iter().max().unwrap();
    let d = |a: &i64, b: &i64| f(*a, i).cmp(&f(*b, i));
    let j = a.into_iter().filter(|a| a != &i).min_by(d).unwrap();
    println!("{} {}", i, j);
}

fn f(a: i64, i: i64) -> i64 {
    ((i + 1) / 2 - a).abs()
}
