use std::ops::{Add, Mul, Sub};
#[derive(Debug)]

#[derive(Clone, Copy, PartialEq)]
struct Matrix<const M: usize, const N: usize> {
    data: [[f64; N]; M],
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl<const M: usize, const N: usize> Matrix<M, N> {
    // This trait requires `fmt` with this exact signature.
    fn print(&self){
        for row in &self.data {
            // let row_str = row.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" ");
            let row_str = row.iter().map(|&x| format!("{:.2}", x)).collect::<Vec<_>>().join(" ");
            
            println!("[{}]", row_str);
        }
    }

    fn transpose(&self)-> Matrix<N, M>{
        let mut result = Matrix {
            data: [[0.; M]; N],
        };
        for i in 0..M{
            for j in 0..N{
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }

    fn max(&self) -> f64 {
        self.data.iter().map(|row| row.iter().copied().fold(f64::NEG_INFINITY, f64::max)).fold(f64::NEG_INFINITY, f64::max)
    }

    fn min(&self) -> f64 {
        self.data.iter().map(|row| row.iter().copied().fold(f64::INFINITY, f64::min)).fold(f64::INFINITY, f64::min)
    }
}

impl<const M: usize, const N: usize> Add<Matrix<M, N>> for Matrix<M, N> {
    type Output = Matrix<M, N>;
    fn add(self, other: Matrix<M, N>) -> Matrix<M, N> {
        let mut result = Matrix {
            data: [[0.; N]; M],
        };
        for i in 0..M{
            for j in 0..N{
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}

impl<const M: usize, const N: usize> Sub<Matrix<M, N>> for Matrix<M, N> {
    type Output = Matrix<M, N>;
    fn sub(self, other: Matrix<M, N>) -> Matrix<M, N> {
        let mut result = Matrix {
            data: [[0.; N]; M],
        };
        for i in 0..M{
            for j in 0..N{
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }
}

impl<const L: usize, const M: usize, const N: usize> Mul<Matrix<M, N>> for Matrix<L, M> {
    type Output = Matrix<L, N>;
    
    fn mul(self, other: Matrix<M, N>) -> Matrix<L, N> {
        let mut result: Matrix<L, N> = Matrix {
            data: [[0.0; N]; L],
        };
        
        for i in 0..L {
            for j in 0..N {
                let mut temp: f64 = 0.0;
                for k in 0..M {
                    temp += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = temp;
            }
        }
        
        result
    }
}

fn linsolve<const M: usize> (mat: Matrix<M, M>, b: Matrix<M,1>) -> Matrix<M,1>{
    let mut result: Matrix<M, 1> = Matrix {
        data: [[0.0; 1]; M],
    };

    return result;
}


fn lu_decomposition<const M: usize> (mat: Matrix<M, M>) -> (Matrix<M, M>, Matrix<M, M>) {
    // Implementation of LU decomposition
    let mut u = mat;
    let mut l = Matrix {
        data: [[0.; M]; M],
    };
    for k in 0..M{
        for i in k..M{
            l.data[i][k] = u.data[i][k]/u.data[k][k];
            // U.print();
            // println!("Division: {}/{} = {}", U.data[i][k], U.data[k][k], U.data[i][k]/U.data[k][k]);
            l.data[i][i] = 1.;
            if i>k{
                for j in 0..M{
                    u.data[i][j] = u.data[i][j] - l.data[i][k]*u.data[k][j];
                }
            }
        }
         
        // println!("-------------L{}-------------", k+1);
        // L.print();
        // println!("-------------U{}-------------", k+1);
        // U.print();
        
    }


    return (l, u)
}

fn forward_substitution<const M: usize>(lower: &Matrix<M, M>, b: &Matrix<M, 1>) -> Matrix<M, 1> {
    // Implementation of forward substitution Ly = b
    let mut y = Matrix{
        data: [[0.; 1];M],
    };
    for i in 0..M{
        y.data[i][0] = b.data[i][0];
        for j in 0..i{
            y.data[i][0] -= lower.data[i][j]*y.data[j][0];
        }
        y.data[i][0] /= lower.data[i][i];
        y.print();
    }
    y
}

fn main() {
    // let mat = Matrix {
    //     data: [[6., 18., 3.], [2., 12., 1.], [4., 15., 3.]],
    // };
    // let b = Matrix {
    //     data: [[1.], [2.], [3.]],
    // };
    let mat = Matrix {
        data: [[2., 1., -1., 2.], [4., 3., -1., 4.], [4., 1., -2., 3.], [6., 3., 1., 7.]],
    };
    let b = Matrix {
        data: [[1.], [2.], [3.], [4.]],
    };
    let (l, u) = lu_decomposition(mat);
    // let res = linsolve(mat, b);
    // L.print();
    // U.print();
    let check = l*u;
    let y = forward_substitution(&l, &b);
    let check2 = l*y;
    check2.print()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    // Define matrices mat and b as static variables using lazy_static
    lazy_static! {
        static ref MAT: Matrix<4, 4> = Matrix {
            data: [[2., 1., -1., 2.], [4., 3., -1., 4.], [4., 1., -2., 3.], [6., 3., 1., 7.]],
        };
        static ref B: Matrix<4, 1> = Matrix {
            data: [[1.], [2.], [3.], [4.]],
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
        assert_eq!(check_forward, *B);
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
}

