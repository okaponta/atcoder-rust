use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        a:[Usize1;n],
    }
    let mut used = vec![false; n];
    used[0] = true;
    let mut tmp = 0;
    let mut count = 0;
    loop {
        count += 1;
        tmp = a[tmp];
        if used[tmp] {
            println!("-1");
            return;
        }
        if tmp == 1 {
            println!("{}", count);
            return;
        }
        used[tmp] = true;
    }
}
