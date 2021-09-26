use proconio::input;

fn main() {
    input!{
        a: i32,
        b: i32,
        k: usize,
    }
    let min:i32;
    let mut gcd = vec![];
    let mut count: i32 = 0;

    if a <= b {
        min = a;
    } else {
        min = b;
    }

    if min == 1 {
        println!("1");
        return;
    }

    for i in 1..min+1 {
        if a % i == 0 && b % i == 0 {
            gcd.push(i);
            count = count + 1;
        }
    }
    gcd.sort();
    gcd.reverse();

    println!("{}", gcd[k-1]);
}