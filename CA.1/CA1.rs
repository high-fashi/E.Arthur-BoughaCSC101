use std::io;

[derive(Debug)]
struct Sibling {
    name: String,
    age: u8,
    status: String,
}

fn main() {
    println!("How many siblings do you have?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_siblings: usize = input.trim().parse().unwrap();

    let mut siblings = Vec::new();

    for _ in 0..num_siblings {
        println!("Enter sibling's name:");
        let name = get_input();

        println!("Enter sibling's age:");
        let age: u8 = get_input().parse().unwrap();

        println!("Enter sibling's status (Married, Single):");
        let status = get_input();

        siblings.push(Sibling { name, age, status });
    }

    println!("Here are the sibling details:");
    println!("{:?}", siblings);
}
fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input 
}