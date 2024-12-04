fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    if let Some(value) = vec.get(1)
    {
        println!("The value is: {}", value);
    }
    else
    {
        println!("Index out of bounds");
    }
}