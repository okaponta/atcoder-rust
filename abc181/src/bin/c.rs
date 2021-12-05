use proconio::input;

fn main() {
    input! {
       n:usize,
       xy: [(i32,i32);n]
    }
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let x1 = xy[j].0 - xy[i].0;
                let x2 = xy[k].0 - xy[i].0;
                let y1 = xy[j].1 - xy[i].1;
                let y2 = xy[k].1 - xy[i].1;
                if x1 * y2 == x2 * y1 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
