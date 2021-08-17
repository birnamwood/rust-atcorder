use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [i32; n],
    }
    let mut count: i32 = 0;
    let mut val: usize = 0;

    for i in 0..a.len() {
        val = a[i] as usize;

        //5番目 = 1 && 1番目 == 5
        if a[val -1] == (i+1) as i32 {
            count += 1;
        }
    }

    println!("{}", count /2);
}