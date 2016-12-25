fn is_prime(n:i32) -> bool{
    if n == 2 {
        return true;
    }
    if n==3 {
        return true;
    }
    if n%2 == 0 {
        return false;
    }
    if n%3 == 0 {
        return false;
    }
    let mut i = 5;
    let mut w = 2;
    while i * i <= n {
        if n%i == 0 {
            return false;
        }
        i+=w;
        w = 6 -w;
    }
    return true;
}

fn main() {
    let mut temp = 0;
    let mut num = 2;
    loop {
        if is_prime(num) {
            temp += 1;
        }
        if temp == 10001 {
            break;
        }
        num += 1;

    }
    println!("{:?}", num);
}
