use proconio::input;

fn main() {
    input! {
        mut a:[usize;3],
    }
    a.sort();
    let mut sub = a[2] * 2 - a[1] - a[0];
    if sub % 2 != 0 {
        sub += 3;
    }
    println!("{}", sub / 2);
}
