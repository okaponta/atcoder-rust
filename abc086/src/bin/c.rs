use proconio::input;

fn main() {
    input! {
        n:usize,
        txy:[(i64,i64,i64);n],
    }
    let mut before = (0, 0, 0);
    for (t, x, y) in txy {
        let d = (x - before.1).abs() + (y - before.2).abs();
        let p = t - before.0;
        if p < d || (p - d) % 2 == 1 {
            println!("No");
            return;
        }
        before = (t, x, y);
    }
    println!("Yes");
}
