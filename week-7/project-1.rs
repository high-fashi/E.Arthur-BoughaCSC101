use std::io;

fn main() {
    println!("Welcome to the Geometry Calculator!");
    println!("Please choose a formula:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let choice = get_input("Enter number choice: ").trim().to_string();

    if choice == "1" {
        calculate_trapezium_area();
    } else if choice == "2" {
        calculate_rhombus_area();
    } else if choice == "3" {
        calculate_parallelogram_area();
    } else if choice == "4" {
        calculate_cube_area();
    } else if choice == "5" {
        calculate_cylinder_volume();
    } else {
        println!("Invalid choice, Restart program.");
    }
}

fn calculate_trapezium_area() {
    let height: f64 = get_input("Enter the height: ").trim().parse().unwrap();
    let base1: f64 = get_input("Enter the first base: ").trim().parse().unwrap();
    let base2: f64 = get_input("Enter the second base: ").trim().parse().unwrap();
    let area = (height / 2.0) * (base1 + base2);
    println!("The area of the trapezium is: {:.2}", area);
}

fn calculate_rhombus_area() {
    let diagonal1: f64 = get_input("Enter the first diagonal: ").trim().parse().unwrap();
    let diagonal2: f64 = get_input("Enter the second diagonal: ").trim().parse().unwrap();
    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {:.2}", area);
}

fn calculate_parallelogram_area() {
    let base: f64 = get_input("Enter the base: ").trim().parse().unwrap();
    let altitude: f64 = get_input("Enter the altitude: ").trim().parse().unwrap();
    let area = base * altitude;
    println!("The area of the parallelogram is: {:.2}", area);
}

fn calculate_cube_area() {
    let side: f64 = get_input("Enter the length of the side: ").trim().parse().unwrap();
    let area = 6.0 * (side * side);
    println!("The surface area of the cube is: {:.2}", area);
}

fn calculate_cylinder_volume() {
    let radius: f64 = get_input("Enter the radius: ").trim().parse().unwrap();
    let height: f64 = get_input("Enter the height: ").trim().parse().unwrap();
    let volume = std::f64::consts::PI * (radius * radius) * height;
    println!("The volume of the cylinder is: {:.2}", volume);
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input!");
    input
}
