use proconio::input;

fn main() {
    input! {
        n: usize,
        pp: [[String;2]; n],
    }
    let mut total = 0;

    for p in pp.iter() {
        let num: i32 = p[1].parse().unwrap();
        total = total + num 
    }
    for p in pp.iter() {
        let num: i32 = p[1].parse().unwrap();
        if num * 2 > total {
            println!("{}", p[0]);
            return;
        }
    }

    println!("{}", "atcorder");
}