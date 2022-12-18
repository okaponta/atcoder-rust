use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        x:i32,
        y:i32,
        mut xi:[i32;n],
        mut yi:[i32;m]
    }
    xi.push(x);
    yi.push(y);
    let diff = yi.into_iter().min().unwrap() - xi.into_iter().max().unwrap();
    println!("{}", if 0 < diff { "No War" } else { "War" });
}
