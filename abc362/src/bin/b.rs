use proconio::input;

fn main() {
    input! {
        a:(i64,i64),
        b:(i64,i64),
        c:(i64,i64),
    }
    let ab = (b.0 - a.0, b.1 - a.1);
    let bc = (c.0 - b.0, c.1 - b.1);
    let ca = (a.0 - c.0, a.1 - c.1);
    if is_ra(ab, bc) || is_ra(bc, ca) || is_ra(ca, ab) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_ra(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 * b.0 + a.1 * b.1 == 0
}
