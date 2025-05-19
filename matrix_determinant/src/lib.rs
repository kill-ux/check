pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    matrix.0 * mat(matrix., n)
}

fn mat(m: (isize, isize), n: (isize, isize)) -> isize {
    (m.0 * n.1) - (m.1 * n.0)
}
