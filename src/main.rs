use std::ops::Add;
#[derive(Debug)]

#[derive(Clone, Copy)] 
struct Matrix<const M: usize, const N: usize> {
    data: [[f64; N]; M],
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl<const M: usize, const N: usize> Matrix<M, N> {
    // This trait requires `fmt` with this exact signature.
    fn print(&self){
        for row in &self.data {
            let row_str = row.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" ");
            println!("[{}]", row_str);
        }
    }

    fn transpose(&self)-> Matrix<M, N>{
        let mut result = Matrix {
            data: [[0.; N]; M],
        };
        for i in 0..M{
            for j in 0..N{
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }

    fn max(&self) -> f64 {
        let mut max = self.data[0][0];

        for row in &self.data {
            for &element in row {
                if element > max {
                    max = element;
                }
            }
        }
        max
    }

    fn min(&self) -> f64 {
        let mut min = self.data[0][0];

        for row in &self.data {
            for &element in row {
                if element < min {
                    min = element;
                }
            }
        }
        min
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


fn main() {
    let matrix1 = Matrix {
        data: [[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]],
    };
    let matrix2 = Matrix {
        data: [[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]],
    };
    let matrix= matrix1 + matrix2.transpose();
    matrix.print();
    println!("Min: {}, Max: {}", matrix2.min(), matrix2.max());
    println!("Min: {}, Max: {}", matrix1.min(), matrix1.max());
    println!("Min: {}, Max: {}", matrix.min(), matrix.max());
}
