fn is_divisible(n: i32, by: i32) -> bool {
    n % by == 0
}

fn check(n: i32) -> bool {
    for i in 1..21 {
        if !is_divisible(n, i) {
            return false;
        }
    }
    return true;
}


fn main() {
    let mut number = 0;
    loop {
        number+=1;
        if check(number) {
            break;
        }
    }
    println!("{:?}", number);
}
