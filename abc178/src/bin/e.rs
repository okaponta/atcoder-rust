use proconio::input;

const MAX: usize = 1_000_000_001;

fn main() {
    input! {
       n:usize,
       xy:[(usize,usize);n],
    }
    let mut min = MAX;
    let mut max = 0;
    let mut rev_min = MAX * 2;
    let mut rev_max = 0;
    for (x, y) in xy {
        min = min.min(x + y);
        max = max.max(x + y);
        rev_min = rev_min.min(x + MAX - y);
        rev_max = rev_max.max(x + MAX - y);
    }
    println!("{}", (max - min).max(rev_max - rev_min));
}
