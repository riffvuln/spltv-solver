pub fn parse_equation(equation: &str) -> (f32, f32, f32, f32) {
    let equation = equation.replace(" ", "");
    let parts: Vec<f32> = equation
        .replace("x", " ")
        .replace("y", " ")
        .replace("z", " ")
        .replace("+", " ")
        .replace("=", " ")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    (parts[0], parts[1], parts[2], parts[3])
}

pub fn equations_to_matrix(equations: &[(f32, f32, f32, f32)]) -> [[f32; 4]; 3] {
    let mut matrix = [[0.0; 4]; 3];
    for (i, eq) in equations.iter().enumerate() {
        matrix[i] = [eq.0, eq.1, eq.2, eq.3];
    }
    matrix
}