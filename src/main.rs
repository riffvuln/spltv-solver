use std::io::{self, Write};

mod gauss;
mod user_interface;

fn main() {
    let mut equations = Vec::new();
    for i in 1..=3 {
        let mut input = String::new();
        print!("Persamaan {}: ", i);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        equations.push(user_interface::parse_equation(&input));
    }

    let matrix = user_interface::equations_to_matrix(&equations);
    println!("matrix = {:?}", matrix);
    match gauss::gauss_elimination(matrix) {
        Some(solution) => println!("solution = {:?}", solution),
        None => println!("No unique solution or infinite solutions"),
    }
}
