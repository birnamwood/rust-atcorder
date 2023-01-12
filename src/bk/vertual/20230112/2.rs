use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }
    let mut total:i32 = 0;

    if x == 1 {
        total += 300000;
    } else if x == 2 {
        total += 200000;
    } else if x == 3 {
        total += 100000;
    }
    if y == 1 {
        total += 300000;
    } else if y == 2 {
        total += 200000;
    } else if y == 3 {
        total += 100000;
    }
    if x == 1 && y == 1 {
        total +=400000;
    }
    println!("{}", total);
}