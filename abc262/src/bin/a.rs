use proconio::input;

fn main() {
    input! {
        mut y:usize,
    }
    loop {
        if y % 4 == 2 {
            println!("{}", y);
            return;
        }
        y += 1;
    }
}
