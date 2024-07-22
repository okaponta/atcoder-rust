use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut min = 1 << 60;
    for i in -100..101 {
        let tmp = a.iter().map(|ai| (ai - i) * (ai - i)).sum::<i64>();
        min = min.min(tmp);
    }
    println!("{}", min);
}
