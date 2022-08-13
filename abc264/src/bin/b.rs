use proconio::input;

fn main() {
    input! {
        mut r:usize,
        mut c:usize,
    }
    r = r.min(16 - r);
    c = c.min(16 - c);
    let mut is_black = false;
    if r == 1 || c == 1 {
        is_black = true;
    }
    if (r == 3 && c != 2) || (c == 3 && r != 2) {
        is_black = true;
    }
    if (r == 5 && c != 2 && c != 4) || (c == 5 && r != 2 && r != 4) {
        is_black = true;
    }
    if (r == 7 && c != 2 && c != 4 && c != 6) || (c == 7 && r != 2 && r != 4 && r != 6) {
        is_black = true;
    }
    println!("{}", if is_black { "black" } else { "white" });
}
