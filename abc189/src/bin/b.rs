use proconio::input;

fn main() {
    input! {
        n:usize,x:i32,
        vp: [(i32,i32);n]
    }
    let mut alc = 0;
    for i in 0..n {
        alc += vp[i].0 * vp[i].1;
        if alc > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
