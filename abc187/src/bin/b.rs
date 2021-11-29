use proconio::input;

fn main() {
    input! {
       n:usize, xy:[(f64,f64);n],
    }
    let mut count = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let slope = (xy[i].1 - xy[j].1) / (xy[i].0 - xy[j].0);
            if -1.0 <= slope && slope <= 1.0 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
