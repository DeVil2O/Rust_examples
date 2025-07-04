fn main() {
    let s = String::from("absba");

    let is_palindrome = is_palindrome(&s);

    println!("{}", is_palindrome)
}

fn is_palindrome(s: &String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    for i in 0..n/2 {
        if chars[i] != chars[n - i - 1] {
            return false
        } 
    }

    return true
}