use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix<const M: usize, const N: usize> {
    pub data: [[f64; N]; M],
}
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl<const M: usize, const N: usize> Matrix<M, N> {
    // This trait requires `fmt` with this exact signature.
    pub fn print(&self){
        for row in &self.data {
            // let row_str = row.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" ");
            let row_str = row.iter().map(|&x| format!("{:.2}", x)).collect::<Vec<_>>().join(" ");
            
            println!("[{}]", row_str);
        }
    }

    pub fn transpose(&self)-> Matrix<N, M>{
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

    pub fn max(&self) -> f64 {
        self.data.iter().map(|row| row.iter().copied().fold(f64::NEG_INFINITY, f64::max)).fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn min(&self) -> f64 {
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

// pub fn linsolve<const M: usize> (mat: Matrix<M, M>, b: Matrix<M,1>) -> Matrix<M,1>{
//     let mut result: Matrix<M, 1> = Matrix {
//         data: [[0.0; 1]; M],
//     };

//     return result;
// }


pub fn lu_decomposition<const M: usize> (mat: Matrix<M, M>) -> (Matrix<M, M>, Matrix<M, M>) {
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

pub fn forward_substitution<const M: usize>(lower: &Matrix<M, M>, b: &Matrix<M, 1>) -> Matrix<M, 1> {
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
    }
    y
}

pub fn backward_substitution<const M: usize>(upper: &Matrix<M, M>, y: &Matrix<M, 1>) -> Matrix<M, 1> {
    // Implementation of forward substitution Ux = y
    let mut x = Matrix{
        data: [[0.; 1];M],
    };
    println!("Upper:");
    upper.print();
    println!("y:");
    y.print();
    for i in (0..M).rev(){
        x.data[i][0] = y.data[i][0];
        for j in (i+1)..M{
            println!("-------------------------------------------");
            println!("j: {}, new = {} - {}*{}",j, x.data[i][0], upper.data[i][j], x.data[j][0]);
            x.data[i][0] -= upper.data[i][j]*x.data[j][0];
            
        }
        println!("{}/{}", x.data[i][0], upper.data[i][i]);
        x.data[i][0] /= upper.data[i][i];
        println!("-------------------------------------------");
        x.print();
    }
    x
}

pub fn linsolve<const M: usize>(mat: &Matrix<M, M>, b: &Matrix<M, 1>) -> Matrix<M, 1> {
    // Implementation of forward substitution Ux = y
    let (l, u) = lu_decomposition(*mat);
    let y = forward_substitution(&l, &b);
    let x = backward_substitution(&u, &y);
    x
}


pub fn all_near <const M: usize, const N: usize> (a: Matrix<M, N>, b: Matrix<M, N>)-> bool{
    let mut result = true;
    a.print();
    b.print();
    for i in 0..M{
        for j in 0..N{
            if (a.data[i][j] - b.data[i][j]).abs() > f64::EPSILON*10.{
                println!("{} - {} > {}", a.data[i][j], b.data[i][j], f64::EPSILON);
                result = false;
            }
        }
    }
    println!("finishd check");
    result
}