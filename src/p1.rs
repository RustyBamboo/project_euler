fn check_if_mult(x: i32) -> bool {
    x % 3 == 0 || x % 5 == 0
}

fn main() {
    let mut count: u64 = 0;
    for i in 0..1000 {
        if check_if_mult(i) {
            count+=i as u64;
        }
    }
    println!("{:?}", count);
}
