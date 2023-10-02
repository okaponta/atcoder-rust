use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        mut a:Usize1,
        mut b:Usize1,
    }
    let mut grid = vec![vec!['#'; 100]; 100];
    for i in 50..100 {
        for j in 0..100 {
            grid[i][j] = '.';
        }
    }
    let mut tmp = 0;
    while 0 < a {
        grid[tmp / 100][tmp % 100] = '.';
        tmp = next(tmp);
        a -= 1;
    }
    tmp = 5100;
    while 0 < b {
        grid[tmp / 100][tmp % 100] = '#';
        tmp = next(tmp);
        b -= 1;
    }
    println!("100 100");
    for i in 0..100 {
        println!("{}", grid[i].iter().join(""));
    }
}

fn next(mut tmp: usize) -> usize {
    tmp += 2;
    if tmp % 100 == 0 {
        tmp += 100;
    }
    tmp
}
