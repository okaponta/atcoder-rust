use proconio::input;

fn main() {
    input! {
        a:[usize;5],
    }
    let mut num = vec![0; 14];
    for ai in a {
        num[ai] += 1;
    }
    num.sort();
    num.dedup();
    if num.len() == 3 && num[1] == 2 && num[2] == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
