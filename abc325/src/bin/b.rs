use proconio::input;

fn main() {
    input! {
        n:usize,
        wx:[(usize,usize);n],
    }
    let mut count = vec![0; 24];
    for (w, x) in wx {
        for i in 9..18 {
            count[(x + i) % 24] += w;
        }
    }
    println!("{}", count.into_iter().max().unwrap());
}
