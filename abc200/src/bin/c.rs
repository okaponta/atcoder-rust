use proconio::input;

fn main() {
    input! {
       n:i32,
       a:[i64;n],
    }
    let mut rem: Vec<i64> = vec![0; 200];
    a.iter().map(|e| e % 200).for_each(|e| rem[e as usize] += 1);
    let ans = rem.iter().fold(0, |acc, x| acc + (x * (x - 1) / 2));
    println!("{}", ans);
}
