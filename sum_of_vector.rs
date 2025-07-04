fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let sum = sum_of_vec(&mut v);
    println!("The sum of the vector is {}", sum);
    println!("Modified vector: {:?}", v);
}

fn sum_of_vec(vec: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in vec.iter() {
        sum += *i;
    }
    vec.push(6);
    sum
}