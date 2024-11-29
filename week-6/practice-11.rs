fn main() {
    let a: i32 = 2; // Binary: 10
    let b: i32 = 3; // Binary: 11

    let mut result: i32;

    result = a & b; // Bitwise AND
    println!("(a & b) -> {}", result);

    result = a | b; // Bitwise OR
    println!("(a | b) -> {}", result);

    result = a ^ b; // Bitwise XOR
    println!("(a ^ b) -> {}", result);

    result = !b; // Bitwise NOT
    println!("(!b) -> {}", result);

    result = a << b; // Bitwise left shift
    println!("(a << b) -> {}", result);

    result = a >> b; // Bitwise right shift
    println!("(a >> b) -> {}", result);
}
