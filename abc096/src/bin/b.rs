use proconio::input;

fn main() {
    input! {
        mut a:[usize;3],
        k:usize,
    }
    a.sort();
    a[2] <<= k;
    println!("{}", a.into_iter().sum::<usize>());
}
