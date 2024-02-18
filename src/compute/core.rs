use graphul::extract::Json;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::error::Error;
use std::vec::Vec;
#[derive(Debug)]
pub struct Matrix {
    n: usize,
    acc: f64,
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
    c: Vec<Vec<f64>>,
    shuffled_matrix: Vec<Vec<f64>>,
    sol: Vec<f64>,
    sol_acc: Vec<f64>,
    sol_iter: usize,
    max_iter: usize,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            n: 0,
            acc: 0.0,
            a: Vec::new(),
            b: Vec::new(),
            c: Vec::new(),
            shuffled_matrix: Vec::new(),
            sol: Vec::new(),
            sol_acc: Vec::new(),
            sol_iter: 0,
            max_iter: 100,
        }
    }

    pub fn init(&mut self, input_string: &str) -> Result<(), Box<dyn Error>> {
        let input_string: Value = serde_json::from_str(input_string)?;
        let input_string = input_string["data"]
            .as_str()
            .ok_or("No 'data' field in input string")?.replace(",",".");
        let mut lines = input_string.lines();

        // Parse the dimension 'n'
        let n_str = lines.next().ok_or("Input string is empty")?;
        self.n = n_str
            .trim()
            .replace(",", ".")
            .parse()
            .map_err(|_| "Invalid input for dimension 'n'")?;

        // Parse matrix coefficients and 'b' values
        for _ in 0..self.n {
            let line = lines.next().ok_or("Insufficient input lines")?;
            let row: Result<Vec<f64>, _> = line
                .split_whitespace()
                .map(|s| {
                    s.replace(",", ".")
                        .parse()
                        .map_err(|_| "Invalid input for matrix coefficients")
                })
                .collect();
            let row = row?;
            let b_val = row.last().ok_or("Invalid input for 'b' value")?;
            self.b.push(*b_val);
            let a_row = row[..self.n].to_vec();
            self.a.push(a_row);
        }

        self.sol = vec![0.0; self.n];
        self.sol_acc = vec![std::f64::MAX; self.n];

        // Parse accuracy
        let acc_str = lines.next().ok_or("Insufficient input lines")?;
        self.acc = acc_str
            .trim()
            .replace(",", ".")
            .parse()
            .map_err(|_| "Invalid input for accuracy")?;
        if self.acc <= 0.0 {
            return Err("Accuracy must be positive".into());
        }

        Ok(())
    }

    pub fn init_from_file(&mut self, file_data: &str) -> Result<(), Box<dyn Error>> {
        let mut lines = file_data.lines();
        self.n = lines.next().unwrap().replace(",", ".").parse().unwrap();

        for _ in 0..self.n {
            let line = lines.next().unwrap();
            let row: Vec<f64> = line
                .split_whitespace()
                .map(|s| s.replace(",", ".").parse().unwrap())
                .collect();
            let b_val = row.last().unwrap();
            self.b.push(*b_val);
            let a_row = row[..self.n].to_vec();
            self.a.push(a_row);
        }

        self.sol = vec![0.0; self.n];
        self.sol_acc = vec![std::f64::MAX; self.n];

        let acc_line = lines.next().unwrap();
        self.acc = acc_line.replace(",", ".").parse().unwrap();
        if self.acc <= 0.0 {
            return Err("Accuracy must be positive".into());
        }

        Ok(())
    }

    fn sum_sol_row(&self, i: usize) -> f64 {
        let mut sum = 0.0;
        for j in 0..self.n {
            if j == i {
                continue;
            }
            sum += self.a[i][j] / self.a[i][i] * self.sol[j];
        }
        sum
    }
    
    fn gaussian_elimination_determinant(mut matrix: Vec<Vec<f64>>) -> f64 {
        let mut det = 1.0;
        let n = matrix.len();
    
        for i in 0..n {
            // Find pivot for column i and swap if necessary
            let mut max = i;
            for j in (i + 1)..n {
                if matrix[j][i].abs() > matrix[max][i].abs() {
                    max = j;
                }
            }
            if matrix[max][i] == 0.0 {
                return 0.0; // Singular matrix, determinant is zero
            }
            if max != i {
                matrix.swap(i, max);
                det *= -1.0; // Swapping rows changes the sign of the determinant
            }
    
            let pivot = matrix[i][i];
            det *= pivot;
    
            // Forward elimination
            for j in (i + 1)..n {
                let factor = matrix[j][i] / pivot;
                for k in i..n {
                    matrix[j][k] -= factor * matrix[i][k];
                }
            }
        }
    
        det
    }
    

    /**
     * Diagonal dominance means that for each row, the magnitude of the diagonal element is greater than
     * the sum of the magnitudes of all the other (non-diagonal) elements in that row.
     */
    fn shuffle(&mut self) -> (bool, Vec<Vec<f64>>) {
        let mut biggest = vec![-1; self.n];
        let mut biggest_set = HashSet::new();
        let mut found_strict = false;

        for i in 0..self.n {
            let sum: f64 = self.a[i].iter().sum();
            for j in 0..self.n {
                if 2.0 * self.a[i][j] >= sum {
                    if 2.0 * self.a[i][j] > sum {
                        found_strict = true;
                    }
                    biggest[i] = j as isize;
                    biggest_set.insert(j);
                    break;
                }
            }
            if biggest[i] == -1 {
                return (false, vec![vec![0.0]]);
            }
        }

        if !found_strict || biggest.len() != biggest_set.len() {
            return (false, vec![vec![0.0]]);
        }

        let mut shuffled_a = vec![vec![]; self.n];
        let mut shuffled_b = vec![0.0; self.n];

        for i in 0..self.n {
            let index = biggest[i] as usize;
            shuffled_a[index] = self.a[i].clone();
            shuffled_b[index] = self.b[i];
        }

        self.a = shuffled_a.clone();
        self.b = shuffled_b;

        (true, shuffled_a)
    }

    fn find_c_and_d(coefficients: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let n = coefficients.len(); // The number of rows, assuming a square matrix for coefficients
        let mut c = vec![vec![0.0; n]; n]; // Initialize C matrix with zeros

        for i in 0..n {
            // Diagonal element of the current row
            let diag_elem = coefficients[i][i];

            for j in 0..n {
                // Check if the current element is not on the diagonal
                if i != j {
                    // C matrix is -1 times the original coefficient matrix divided by the diagonal element
                    c[i][j] = -coefficients[i][j] / diag_elem;
                }
            }
            // The diagonal elements of C are set to zero
            c[i][i] = 0.0;
        }

        c
    }

    fn iterate(&mut self) {
        let mut new_sol = vec![0.0; self.n];
        for i in 0..self.n {
            new_sol[i] = self.b[i] / self.a[i][i] - self.sum_sol_row(i);
            self.sol_acc[i] = (new_sol[i] - self.sol[i]).abs();
        }
        self.sol = new_sol;
        self.sol_iter += 1;
    }

    pub fn solve(&mut self) -> Json<serde_json::Value> {
        let mut err = String::new();

        if Self::gaussian_elimination_determinant(self.a.clone()) == 0.0 {
            return Json(json!({"error": "Singular matrix"}));
        }

        if !self.shuffle().0 {
            err = String::from("Невозможно привести к диагональному преобладанию.")
        }

        self.shuffled_matrix = self.shuffle().1;

        while self.sol_acc.iter().max_by(|a, b| a.total_cmp(b)).unwrap() > &self.acc
            && self.sol_iter < self.max_iter
        {
            self.iterate();
        }
        // self.print_sol();

        if !self.shuffled_matrix.is_empty() {
            self.c = Matrix::find_c_and_d(self.shuffle().1);
            return Json(json!({
                "sol": self.sol,
                "acc": self.sol_acc,
                "iter": self.sol_iter,
                "c": self.c,
                "mtrx": self.shuffled_matrix,
                "err": err,
            }));
        }

        Json(json!({
            "sol": self.sol,
            "acc": self.sol_acc,
            "iter": self.sol_iter,
            "mtrx": self.shuffled_matrix,
            "err": err,
        }))
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.n {
            for j in 0..self.n {
                print!("{} ", self.a[i][j]);
            }
            println!("| {}", self.b[i]);
        }
    }

    #[allow(dead_code)]
    fn print_sol(&self) {
        println!("--- Решение ---");
        for i in 0..self.n {
            println!("x{}: {} (delta = {})", i + 1, self.sol[i], self.sol_acc[i]);
        }
        println!("Найдено за {} итераций.", self.sol_iter);
        println!("array {:?}", self)
    }
}
