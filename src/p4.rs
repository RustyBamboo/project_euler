fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    for (a,b) in s.chars().zip(s.chars().rev()) {
        if a!=b {
            return false;
        }
    }
    return true;
}


fn main() {
    let mut largest = 0;
    for i in 100..999 {
        for j in 100..999{
             if is_palindrome(i * j) && ((i * j) > largest) {
                largest = i * j;
                println!("Largest: {}, {} x {}", largest, i, j);
            }
        }
    }
}
