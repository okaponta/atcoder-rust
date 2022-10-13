use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let first = s[..2].parse().unwrap();
    let last = s[2..].parse().unwrap();
    if is_mm(first) {
        println!("{}", if is_mm(last) { "AMBIGUOUS" } else { "MMYY" });
    } else {
        println!("{}", if is_mm(last) { "YYMM" } else { "NA" });
    }
}

fn is_mm(i: i32) -> bool {
    0 < i && i < 13
}
