use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a:[[u16;3];3],
        n:usize,
        b:[u16;n],
    }
    let set = b.into_iter().collect::<HashSet<u16>>();
    let mut called = vec![vec![false; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            if set.contains(&a[i][j]) {
                called[i][j] = true;
            }
        }
    }
    if (called[0][0] && called[0][1] && called[0][2])
        || (called[1][0] && called[1][1] && called[1][2])
        || (called[2][0] && called[2][1] && called[2][2])
        || (called[0][0] && called[1][0] && called[2][0])
        || (called[0][1] && called[1][1] && called[2][1])
        || (called[0][2] && called[1][2] && called[2][2])
        || (called[0][0] && called[1][1] && called[2][2])
        || (called[2][0] && called[1][1] && called[0][2])
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
