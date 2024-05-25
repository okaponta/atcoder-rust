use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        t:usize,
        a:[Usize1;t],
    }
    let mut row = vec![0; n];
    let mut col = vec![0; n];
    let mut x = vec![0; 2];
    for (i, a) in a.into_iter().enumerate() {
        let ri = a % n;
        let ci = a / n;
        row[ri] += 1;
        col[ci] += 1;
        if ri == ci {
            x[0] += 1;
        }
        if ri + ci == n - 1 {
            x[1] += 1;
        }
        if row[ri] == n || col[ci] == n || x[0] == n || x[1] == n {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
