use proconio::input;

fn main() {
    input! {
       n:String,
    }
    let sum: i32 = n.chars().map(|x| x as i32).sum();
    let digits = n.chars().map(|x| x as i32).count();
    let rem_1 = n.chars().map(|x| x as i32).filter(|&x| x % 3 == 1).count();
    let rem_2 = n.chars().map(|x| x as i32).filter(|&x| x % 3 == 2).count();
    let mut count: i32 = 0;
    if sum % 3 == 0 {
    } else if sum % 3 == 1 {
        if rem_1 > 0 {
            count = 1;
        } else if rem_2 > 1 {
            count = 2
        } else {
            count = -1;
        }
    } else if sum % 3 == 2 {
        if rem_2 > 0 {
            count = 1;
        } else if rem_1 > 1 {
            count = 2
        } else {
            count = -1;
        }
    }
    if digits <= count as usize {
        count = -1;
    }
    println!("{}", count);
}
