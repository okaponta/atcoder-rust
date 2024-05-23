use proconio::input;

fn main() {
    input! {
        h:usize,
    }
    let mut i = 1;
    while 2usize.pow(i) - 1 <= h {
        i += 1;
    }
    println!("{}", i);
}
