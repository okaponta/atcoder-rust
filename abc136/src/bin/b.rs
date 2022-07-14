use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = 0;
    for mut i in 1..=n {
        let mut count = 1;
        while i / 10 != 0 {
            i /= 10;
            count += 1;
        }
        if count % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
