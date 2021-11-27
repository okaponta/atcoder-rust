use proconio::input;

fn main() {
    input! {
       a:i32, b:i32, w:i32,
    }
    let mut min_count: i32 = 0;
    let mut max_count: i32 = 0;
    for i in 1..w * 1000 / a + 2 {
        if a * i <= w * 1000 && w * 1000 <= b * i {
            if min_count == 0 {
                min_count = i;
            }
            max_count = i;
        }
    }
    if min_count == 0 {
        println!("UNSATISFIABLE");
        return;
    }
    println!("{} {}", min_count, max_count);
}
