use proconio::input;
use itertools::Itertools;

fn main() {
    input!{
        n: usize,
        //a: [[i32; 2]; n],
    }
    let mut cnt = n;
    let mut flg = false;
    // nが奇数であればn -1
    if n % 2 == 1 {
        flg = true;
        cnt = cnt - 1;
    }
    cnt = cnt / 2;

    for i in 0..cnt {
        for j in 0..cnt {
            if i == j {
                return;
            }
            println!("{}", i);
        }
    }

}