use proconio::input;

fn main() {
    input! {
       n:i32,q:i32,
    }
    let mut train = (0..=n).into_iter().map(|_e| (0, 0)).collect::<Vec<_>>();
    for _i in 0..q {
        input! {
           qq:i32,
        }
        if qq == 1 {
            input! { x:usize,y:usize, }
            let x_af = (train[x].0, y as i32);
            let y_af = (x as i32, train[y].1);
            train[x] = x_af;
            train[y] = y_af;
        }
        if qq == 2 {
            input! { x:usize,y:usize, }
            let x_af = (train[x].0, 0);
            let y_af = (0, train[y].1);
            train[x] = x_af;
            train[y] = y_af;
        }
        if qq == 3 {
            input! { x:usize,}
            let mut first = x as i32;
            let mut tmp = train[x];
            while tmp.0 != 0 {
                first = tmp.0;
                tmp = train[tmp.0 as usize];
            }
            let mut out = vec![first];
            while tmp.1 != 0 {
                out.push(tmp.1);
                tmp = train[tmp.1 as usize];
            }
            println!(
                "{} {}",
                out.len(),
                out.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }
}
