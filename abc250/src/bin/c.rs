use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        x:[Usize1;q],
    }
    let mut num = vec![];
    let mut index = vec![];
    for i in 0..n {
        num.push(i);
        index.push(i);
    }
    for xi in x {
        let idx_one = index[xi];
        let idx_two = if idx_one == n - 1 {
            idx_one - 1
        } else {
            idx_one + 1
        };
        let num_one = num[idx_one];
        let num_two = num[idx_two];
        num[idx_one] = num_two;
        num[idx_two] = num_one;
        index[num_one] = idx_two;
        index[num_two] = idx_one;
    }
    println!("{}", num.iter().map(|i| (i + 1).to_string()).join(" "));
}
