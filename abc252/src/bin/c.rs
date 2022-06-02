use std::vec;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut indexes = vec![vec![0; n]; 10];
    for i in 0..n {
        for j in 0..10 {
            let num = s[i][j].to_digit(10).unwrap() as usize;
            indexes[num][i] = j;
        }
    }

    let mut min = 1 << 60;
    for i in 0..10 {
        let mut ans = vec![0; 10];
        for j in 0..n {
            ans[indexes[i][j]] += 1;
        }
        let mut temp = 0;
        for j in 0..10 {
            temp = temp.max(ans[j] * 10 + j);
        }
        min = min.min(temp - 10);
    }
    println!("{}", min);
}
