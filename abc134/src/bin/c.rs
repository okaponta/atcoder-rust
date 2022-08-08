use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut a:[usize;n]
    }
    let mut max = 0;
    let mut index = 0;
    for i in 0..n {
        if max < a[i] {
            index = i;
            max = a[i];
        }
    }
    a.sort();
    for i in 0..n {
        if i == index {
            println!("{}", a[n - 2]);
        } else {
            println!("{}", a[n - 1]);
        }
    }
}
