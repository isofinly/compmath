use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;


#[derive(Debug)]
pub struct Matrix {
    n: usize,
    acc: f64,
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
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
            sol: Vec::new(),
            sol_acc: Vec::new(),
            sol_iter: 0,
            max_iter: 100,
        }
    }

    pub fn init(&mut self, input_string: &str) {
        let mut lines = input_string.lines();

        // Parse the dimension 'n'
        let n_str = lines.next().expect("Input string is empty");
        self.n = n_str.trim().parse().expect("Invalid input for dimension 'n'");

        // Parse matrix coefficients and 'b' values
        for _ in 0..self.n {
            let line = lines.next().expect("Insufficient input lines");
            let row: Vec<f64> = line.split_whitespace().map(|s| s.parse().expect("Invalid input for matrix coefficients")).collect();
            let b_val = row.last().expect("Invalid input for 'b' value").clone();
            self.b.push(b_val);
            let a_row = row[..self.n].to_vec();
            self.a.push(a_row);
        }

        self.sol = vec![0.0; self.n];
        self.sol_acc = vec![std::f64::MAX; self.n];

        // Parse accuracy
        let acc_str = lines.next().expect("Insufficient input lines");
        self.acc = acc_str.trim().parse().expect("Invalid input for accuracy");
    }

    pub fn init_from_file(&mut self, file_path: &str) -> Result<(), io::Error> {
        let file = File::open(file_path)?;
        let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());

        self.n = lines.next().unwrap().parse().unwrap();

        for _ in 0..self.n {
            let line = lines.next().unwrap();
            let row: Vec<f64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            let b_val = row.last().unwrap().clone();
            self.b.push(b_val);
            let a_row = row[..self.n].to_vec();
            self.a.push(a_row);
        }

        self.sol = vec![0.0; self.n];
        self.sol_acc = vec![std::f64::MAX; self.n];

        let acc_line = lines.next().unwrap();
        self.acc = acc_line.parse().unwrap();

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

    fn shuffle(&mut self) -> bool {
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
                return false;
            }
        }

        if !found_strict || biggest.len() != biggest_set.len() {
            return false;
        }

        let mut shuffled_a = vec![vec![]; self.n];
        let mut shuffled_b = vec![0.0; self.n];
        for i in 0..self.n {
            let index = biggest[i] as usize;
            shuffled_a[index] = self.a[i].clone();
            shuffled_b[index] = self.b[i];
        }

        self.a = shuffled_a;
        self.b = shuffled_b;
        true
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

    fn solve(&mut self) {
        if !self.shuffle() {
            println!("Невозможно привести к диагональному преобладанию.");
            return;
        }
        while self.sol_acc.iter().max_by(|a, b| a.total_cmp(b)).unwrap() > &self.acc && self.sol_iter < self.max_iter {
            self.iterate();
        }
        self.print_sol();
    }

    fn print(&self) {
        for i in 0..self.n {
            for j in 0..self.n {
                print!("{} ", self.a[i][j]);
            }
            println!("| {}", self.b[i]);
        }
    }

    fn print_sol(&self) {
        println!("--- Решение ---");
        for i in 0..self.n {
            println!("x{}: {} (delta = {})", i + 1, self.sol[i], self.sol_acc[i]);
        }
        println!("Найдено за {} итераций.", self.sol_iter);
        println!("array {:?}", self)
    }
}

