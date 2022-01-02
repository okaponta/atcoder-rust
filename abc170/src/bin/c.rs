use proconio::input;

fn main() {
    input! {
       x:i32,n:i32,
       p:[i32;n],
    }
    for i in 0..101 {
        if !p.contains(&(x - i)) {
            println!("{}", x - i);
            return;
        }
        if !p.contains(&(x + i)) {
            println!("{}", x + i);
            return;
        }
    }
}
