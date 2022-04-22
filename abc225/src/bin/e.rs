use proconio::input;

fn main() {
    input! {
       n:usize,
       mut xy:[(i64,i64);n],
    }
    xy.sort_by(|a, b| ((a.0 * (b.1 - 1)).cmp(&((a.1 - 1) * b.0))));
    let mut cur = (0, 1);
    let mut count = 0;
    for (x, y) in xy {
        if cur.0 * y <= cur.1 * (x - 1) {
            count += 1;
            cur = (x, y - 1);
        }
    }
    println!("{}", count);
}
