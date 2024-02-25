
#[cfg(test)]
mod tests {
    use linalg::matrix::*;
    use lazy_static::lazy_static;

    // Define matrices mat and b as static variables using lazy_static
    lazy_static! {
        static ref MAT: Matrix<4, 4> = Matrix {
            data: [[2., 1., -1., 2.], [4., 3., -1., 4.], [4., 1., -2., 3.], [6., 3., 1., 7.]],
        };
        static ref B: Matrix<4, 1> = Matrix {
            data: [[1.], [2.], [3.], [4.]],
        };
        static ref MAT_TRANSPOSED: Matrix<4, 4> = Matrix {
            data: [[2., 4., 4., 6.], [1., 3., 1., 3.], [-1., -1., -2., 1.], [2., 4., 3., 7.]],
        };
    }

    #[test]
    fn test_lu_decomposition() {
        // Decompose matrix to LU
        let (l, u) = lu_decomposition(*MAT);

        // Check that L*U == mat
        let check_lu = l * u;
        assert_eq!(check_lu, *MAT);
    }

    #[test]
    fn test_forward_substitution() {
        // Perform forward substitution
        let (l, _) = lu_decomposition(*MAT);
        let y = forward_substitution(&l, &*B);

        // Check that L*y == b
        let check_forward = l * y;
        assert!(all_near(check_forward, *B));
    }

    #[test]
    fn test_backward_substitution() {
        // Perform forward substitution
        let (l, u) = lu_decomposition(*MAT);
        let y = forward_substitution(&l, &*B);
        let x = backward_substitution(&u, &y);
        // Check that U*x = y
        let check_backward = u * x;
        assert!(all_near(check_backward, y));
    }

    #[test]
    fn test_linsolve() {
        // Solve linear system of equations
        let x = linsolve(&*MAT, &*B);
        // Check if solution fulfills system of equations
        let check_solution = *MAT*x;
        assert!(all_near(check_solution, *B));
    }


    #[test]
    fn test_min_max() {
        // check min
        let min = MAT.min();
        assert_eq!(min, -2.);
        // check max
        let max = MAT.max();
        assert_eq!(max, 7.);        
    }
    #[test]
    fn test_transpose() {
        let transposed = MAT.transpose();
        assert_eq!(transposed, *MAT_TRANSPOSED);
    }
}

