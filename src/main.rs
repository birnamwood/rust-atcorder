use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }
    let mut total: i32 = 0;

    total += add(x);
    total += add(y);

    if x == 1 && y == 1 {
        total += 400000;
    }
    println!("{}", total);
}

fn add(target: i32) -> i32 {
    if target == 1 {
        return 300000;
    } else if target == 2 {
        return 200000;
    } else if target == 3 {
        return 100000;
    }
    return 0;
}
