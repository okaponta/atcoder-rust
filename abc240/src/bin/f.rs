use proconio::input;

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        testcase();
    }
}

fn testcase() {
    input! {
        n:usize,_:usize,
        xy:[(i64,i64);n],
    }
    let mut ans = xy[0].0;
    let mut prev_b = 0;
    let mut prev_a = 0;
    let mut prev_xy_sum = 0;
    for (x, y) in xy {
        let next_b = prev_b + x * y;
        let next_a = prev_a + prev_xy_sum * y + y * (y + 1) * x / 2;
        let next_xy_sum = prev_xy_sum + x * y;
        if prev_b > 0 && next_b < 0 {
            let tmp = prev_b / x.abs();
            let tmp_a = prev_a + prev_xy_sum * tmp + tmp * (tmp + 1) * x / 2;
            ans = ans.max(tmp_a);
        }
        prev_b = next_b;
        prev_a = next_a;
        prev_xy_sum = next_xy_sum;
        ans = ans.max(next_a);
    }
    println!("{}", ans);
}
