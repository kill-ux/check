pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    let res1 = matrix[0][0] * mat((matrix[1][1], matrix[1][2]), (matrix[2][1], matrix[2][2]));
    let res2 = matrix[0][1] * mat((matrix[1][0], matrix[1][2]), (matrix[2][0], matrix[2][2]));
    let res3 = matrix[0][2] * mat((matrix[1][0], matrix[1][1]), (matrix[2][0], matrix[2][1]));
    res1 - res2 + res3
}

fn mat(m: (isize, isize), n: (isize, isize)) -> isize {
    (m.0 * n.1) - (m.1 * n.0)
}
