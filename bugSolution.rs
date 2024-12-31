fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;

    // Safe way to access vector elements
    match vec.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    };

    //Alternative using if-let
    if let Some(value) = vec.get(index){
        println!("Value at index {}: {}", index, value);
    } else {
        println!("Index {} is out of bounds",index);
    }
}