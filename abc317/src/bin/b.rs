use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    for i in 1..n {
        if a[i] - a[i - 1] != 1 {
            println!("{}", a[i - 1] + 1);
            return;
        }
    }
}
