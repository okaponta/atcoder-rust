use proconio::input;

fn main() {
    input! {
        n:usize,
        mut ab:[(usize,usize);n],
    }
    ab.sort_by(|a, b| a.1.cmp(&b.1));
    let mut t = 0;
    for (a, b) in ab {
        t += a;
        if b < t {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
