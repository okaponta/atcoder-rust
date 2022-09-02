use proconio::input;

fn main() {
    input! {
        a:(i32,i32),
        b:(i32,i32),
        c:(i32,i32),
        d:(i32,i32),
    }
    let ab = (b.0 - a.0, b.1 - a.1);
    let bc = (c.0 - b.0, c.1 - b.1);
    let cd = (d.0 - c.0, d.1 - c.1);
    let da = (a.0 - d.0, a.1 - d.1);
    let ipa = inner_product(ab.0, bc.0, ab.1, bc.1);
    let ipb = inner_product(bc.0, cd.0, bc.1, cd.1);
    let ipc = inner_product(cd.0, da.0, cd.1, da.1);
    let ipd = inner_product(da.0, ab.0, da.1, ab.1);
    println!(
        "{}",
        if 0 <= ipa && 0 <= ipb && 0 <= ipc && 0 <= ipd {
            "Yes"
        } else {
            "No"
        }
    );
}

// 2つのベクトルの内積を返却
// これが負なら、ベクトルのなす角は90度以上
fn inner_product(x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    x0 * x1 + y0 * y1
}
