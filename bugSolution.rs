fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = vec[0]; // No longer a reference
    vec.push(3);
    println!("{}", x); 
}