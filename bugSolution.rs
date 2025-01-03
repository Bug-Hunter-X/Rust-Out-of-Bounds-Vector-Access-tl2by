fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Check if the index is valid before accessing the element
    if vec.len() > 2 {
        let third = vec[2];
        println!("The third element is {}", third);
    } else {
        println!("The vector does not have a third element.");
    }
} 