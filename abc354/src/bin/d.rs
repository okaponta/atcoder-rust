use proconio::input;

fn main() {
    input! {
        mut a:i64,
        mut b:i64,
        mut c:i64,
        mut d:i64,
    }
    if a < 0 {
        let off_x = (-a / 4) * 4 + 4;
        a += off_x;
        c += off_x;
    }
    if b < 0 {
        let off_y = (-b / 4) * 4 + 4;
        b += off_y;
        d += off_y;
    }
    let a = a as usize;
    let b = b as usize;
    let c = c as usize;
    let mut d = d as usize;
    let base = vec![vec![2, 1, 0, 1], vec![1, 2, 1, 0]];
    let mut ans = 0;
    if (d - b) % 2 == 1 {
        ans += ((c - a) / 4) * 4;
        for i in 0..(c - a) % 4 {
            ans += base[b % 2][(a + i) % 4];
        }
        d -= 1;
    }
    let b2 = vec![3, 3, 1, 1];
    ans += ((c - a) / 4) * 4 * (d - b);
    for i in 0..(c - a) % 4 {
        ans += b2[(a + i) % 4] * (d - b) / 2;
    }
    println!("{}", ans);
}
