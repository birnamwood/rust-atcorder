use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    if (a * a) + (b * b) < (c * c) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main2() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    let mut x = 0;
    let mut y = 0;
    //aの最大値
    for i in a.iter() {
        if x < *i {
            x = *i;
        }
    }
    //bの最小値
    for j in b.iter() {
        if y == 0 {
            y = *j;
        }
        if y > *j {
            y = *j;
        }
    }
    // println!("{}", x);
    // println!("{}", y);
    if y -x + 1 <= 0 {
        println!("{}", 0);
    } else {
        println!("{}", y - x + 1);
    }

}