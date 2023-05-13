use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut result = Vec::new();

    for i in 0..n - 1 {
        result.push(a[i]);
        if (a[i] - a[i + 1]).abs() != 1 {
            if a[i] < a[i + 1] {
                for j in (a[i] + 1)..a[i + 1] {
                    result.push(j);
                }
            } else {
                for j in (a[i + 1] + 1..(a[i])).rev() {
                    result.push(j);
                }
            }
        }
    }

    result.push(a[n - 1]);
    println!("{}", result.iter().join(" "));
}
