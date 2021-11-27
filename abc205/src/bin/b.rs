use proconio::input;

fn main() {
    input! {
            n: usize,
            mut a: [usize;n],
    }
    a.sort();
    let mut res = "Yes";
    for i in 0..n {
        if a[i] != i + 1 {
            res = "No";
            break;
        }
    }
    println!("{}", res);
}
