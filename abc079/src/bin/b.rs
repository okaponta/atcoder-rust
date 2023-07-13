use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut lucas = vec![1usize; 87];
    lucas[0] = 2;
    for i in 2..=86 {
        lucas[i] = lucas[i - 1] + lucas[i - 2];
    }
    println!("{}", lucas[n]);
}
