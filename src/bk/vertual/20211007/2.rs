use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
        c: [i32; n -1],
    }
    let mut point: i32 = 0;
    let mut prev: i32 = 0;

    for i in 0..(n) {
        point = point + b[(a[i] - 1) as usize];

        
        if prev != 0 && prev + 1 == a[i] {
            point = point +  (c[(a[i] - 2) as usize]);
        }
        prev = a[i];
    }

    println!("{}", point);
}