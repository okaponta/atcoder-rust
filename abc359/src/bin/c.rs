use proconio::input;

fn main() {
    input! {
        s:(i64,i64),
        t:(i64,i64),
    }
    let y = (s.1 - t.1).abs();
    if (s.0 + s.1) % 2 == 0 {
        // 左
        let min = s.0 - y;
        let max = s.0 + 1 + y;
        if min <= t.0 && t.0 <= max {
            println!("{}", y);
        } else if t.0 < min {
            println!("{}", y + (min + 1 - t.0) / 2);
        } else {
            println!("{}", y + (t.0 - max + 1) / 2);
        }
    } else {
        let min = s.0 - y - 1;
        let max = s.0 + y;
        if min <= t.0 && t.0 <= max {
            println!("{}", y);
        } else if t.0 < min {
            println!("{}", y + (min + 1 - t.0) / 2);
        } else {
            println!("{}", y + (t.0 - max + 1) / 2);
        }
    }
}
