use proconio::input;

fn main() {
    input! {
        p:char,
        q:char,
    }
    println!("{}", (d(p) - d(q)).abs())
}

fn d(c: char) -> i32 {
    return match c {
        'A' => 0,
        'B' => 3,
        'C' => 4,
        'D' => 8,
        'E' => 9,
        'F' => 14,
        _ => 23,
    };
}
