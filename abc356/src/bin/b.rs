use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
        x:[[usize;m];n],
    }
    let mut s = vec![0; m];
    for i in 0..n {
        for j in 0..m {
            s[j] += x[i][j];
        }
    }
    for i in 0..m {
        if s[i] < a[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
