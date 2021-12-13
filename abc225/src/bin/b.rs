use proconio::input;

fn main() {
    input! {
       n:usize,
       ab:[usize;2*n-2]
    }
    let mut count = vec![0; n];
    ab.iter().for_each(|&x| count[x - 1] += 1);
    println!(
        "{}",
        if count.iter().any(|&e| e == n - 1) {
            "Yes"
        } else {
            "No"
        }
    );
}
