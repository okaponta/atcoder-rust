use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        a:[Usize1;4*n-1],
    }
    let mut map = vec![0; n];
    for ai in a {
        map[ai] += 1;
    }
    for i in 0..n {
        if map[i] == 3 {
            println!("{}", i + 1);
        }
    }
}
