fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let value = vec.get(10).unwrap();
    println!("The value is: {}", value);
}