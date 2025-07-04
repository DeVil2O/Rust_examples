fn main() {
    let n = 10;
    let result = factorial(n);
    println!("The factorial of {} is {}", n, result);

}

pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}