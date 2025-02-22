fn eliminate(mat: &mut [[f32; 4]; 3], row: usize, col: usize) -> bool {
    if mat[row][col] == 0.0 {
        return false;
    }
    for i in (row + 1)..mat.len() {
        let factor = mat[i][col] / mat[row][col];
        for j in col..mat[i].len() {
            mat[i][j] -= factor * mat[row][j];
        }
    }
    true
}

pub fn gauss_elimination(mut mat: [[f32; 4]; 3]) -> Option<Vec<f32>> {
    let num_variables = mat.len();
    for i in 0..num_variables {
        if !eliminate(&mut mat, i, i) {
            return None;
        }
    }

    for i in (0..num_variables).rev() {
        if mat[i][i].abs() < 1e-6 {
            if mat[i][num_variables].abs() < 1e-6 {
                return None;
            } else {
                return None;
            }
        }
    }

    let mut solution = vec![0.0; num_variables];
    for i in (0..num_variables).rev() {
        let mut sum = mat[i][num_variables];
        for j in (i + 1)..num_variables {
            sum -= mat[i][j] * solution[j];
        }
        solution[i] = sum / mat[i][i];
    }
    Some(solution)
}
