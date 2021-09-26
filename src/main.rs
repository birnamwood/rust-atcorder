use proconio::input;

fn main() {
    input!{
        k: i64,
        a: i64,
        b: i64,
    }
    let ans:i64 = change(k, a) * change(k, b);

    println!("{}", ans);
}

// x n進数
fn change(mut a: i64,x: i64)->i64{
    let mut amari: Vec<i64> = Vec::new();
    while x != 0{
        amari.push(x % a);
        x /= a;
    }
    let mut n:i64 = 0;
    for i in 0..amari.len(){
         n += 10i64.pow(i as u32)*amari[i]
    }
    return n;
}