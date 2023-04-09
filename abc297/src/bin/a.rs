use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        t: [i32; n],
    }

    let mut last_click_time = -1;
    let mut double_click_time = -1;

    for i in 1..n {
        if t[i] - t[i - 1] <= d {
            if last_click_time == -1 {
                last_click_time = t[i - 1];
            }
            double_click_time = t[i];
        } else {
            last_click_time = -1;
        }
        if double_click_time != -1 && double_click_time - last_click_time <= d {
            println!("{}", double_click_time);
            return;
        }
    }

    println!("-1");
}
