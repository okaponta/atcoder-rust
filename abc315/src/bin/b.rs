use proconio::input;

fn main() {
    input! {
        m:usize,
        d:[usize;m],
    }
    let mut mid = d.iter().sum::<usize>() / 2;
    for i in 0..m {
        if mid < d[i] {
            println!("{} {}", i + 1, mid + 1);
            return;
        }
        mid -= d[i];
    }
}
