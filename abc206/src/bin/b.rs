use proconio::input;

fn main() {
    input! {
            n: i32
    }
    let mut count = 0;
    let mut save = 0;
    while save < n {
        count += 1;
        save += count
    }
    println!("{}", count);
}
