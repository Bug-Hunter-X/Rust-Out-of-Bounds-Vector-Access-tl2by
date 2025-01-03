fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let third = vec[2]; //This line will cause a panic because we try to access an index that is out of bounds
    println!("The third element is {}", third);
}