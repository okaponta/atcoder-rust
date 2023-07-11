use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut ab:[(usize,usize);n],
    }
    ab.sort();
    ab.reverse();
    let mut s = 0;
    for (a, b) in ab {
        s += b;
        if k < s {
            println!("{}", a + 1);
            return;
        }
    }
    println!("1");
}
