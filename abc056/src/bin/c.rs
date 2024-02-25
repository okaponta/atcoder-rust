use proconio::input;

fn main() {
    input! {
        x:usize,
    }
    let mut max = 0;
    let mut i = 1;
    while max < x {
        max += i;
        i += 1;
    }
    println!("{}", i - 1);
}
