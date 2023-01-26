use proconio::input;

fn main() {
    input! {
        x: i32,
        t: i32,
    }
    let mut zan: i32 = 0;
    if x - t > 0 {
        zan = x - t;
    }

    println!("{}", zan);
}
