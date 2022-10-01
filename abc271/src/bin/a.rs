use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let ans = format!("{:x}", n);
    let ans = ans.to_uppercase();
    if ans.len() == 1 {
        println!("0{}", ans);
    } else {
        println!("{}", ans);
    }
}
