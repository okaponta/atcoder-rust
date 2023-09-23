use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        a:[usize;n-1],
    }
    for i in 0..=100 {
        let mut b = a.clone();
        b.push(i);
        b.sort();
        let mut sum = 0;
        for j in 1..n - 1 {
            sum += b[j];
        }
        if x <= sum {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
