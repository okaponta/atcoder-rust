use proconio::input;

fn main() {
    input! {
       n:usize,
       d:[(i32,i32);n]
    }
    let mut count = 0;
    for i in 0..n {
        if d[i].0 == d[i].1 {
            count += 1;
            if count == 3 {
                println!("Yes");
                return;
            }
        } else {
            count = 0;
        }
    }
    println!("No");
}
