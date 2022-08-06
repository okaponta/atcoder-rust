use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        mut p:[Usize1;n-1],
    }
    p.insert(0, 0);
    let mut temp = n - 1;
    let mut count = 0;
    while temp != 0 {
        temp = p[temp];
        count += 1;
    }
    println!("{}", count);
}
