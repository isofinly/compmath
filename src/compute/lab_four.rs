use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Function {
    Polynomial(u16),
    Exponential,
    Logarithmic,
    Power,
}

pub struct ApproximationCalculator {
    function: Function,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub coefficients: Vec<f64>,
}

const ACCURACY: f64 = 0.001;

impl ApproximationCalculator {
    pub fn new(function: Function, x: Vec<f64>, y: Vec<f64>) -> Self {
        ApproximationCalculator {
            function,
            x,
            y,
            coefficients: Vec::new(),
        }
    }

    pub fn calculate_coefficients(&mut self) -> Vec<f64> {
        let approximations =
            approximation_calculation(self.function, self.x.len(), &self.x, &self.y);
        self.coefficients = approximations.clone();
        approximations
    }

    pub fn calculate_differences(&self) -> Vec<f64> {
        differences_calculation(
            self.function,
            self.x.len(),
            &self.coefficients,
            &self.x,
            &self.y,
        )
    }

    pub fn calculate_standard_deviation(&self) -> f64 {
        standard_deviation_calculation(
            &differences_calculation(
                self.function,
                self.x.len(),
                &self.coefficients,
                &self.x,
                &self.y,
            ),
            self.x.len(),
        )
    }

    pub fn calculate_pearson_correlation(&self) -> Result<f64, &str> {
        let n = self.x.len() as f64;
        let sum_x = self.x.iter().sum::<f64>();
        let sum_y = self.y.iter().sum::<f64>();
        let sum_xy = self
            .x
            .iter()
            .zip(self.y.iter())
            .map(|(x, y)| x * y)
            .sum::<f64>();
        let sum_x_squared = self.x.iter().map(|x| x * x).sum::<f64>();
        let sum_y_squared = self.y.iter().map(|y| y * y).sum::<f64>();

        let numerator = n * sum_xy - sum_x * sum_y;
        let denominator =
            ((n * sum_x_squared - sum_x * sum_x) * (n * sum_y_squared - sum_y * sum_y)).sqrt();

        if denominator == 0.0 {
            return Err("Division by zero (деление на ноль)");
        }

        let r = numerator / denominator;

        if r.abs() < 0.8 {
            return Err("No strong linear dependency (линейная зависимость) detected.");
        }

        Ok(r)
    }

    pub fn get_phi_values(&self) -> Vec<f64> {
        self.x
            .iter()
            .map(|&x_i| get_function_value(self.function, &self.coefficients, x_i))
            .collect()
    }

    pub fn get_epsilon_values(&self) -> Vec<f64> {
        self.x
            .iter()
            .zip(self.y.iter())
            .map(|(&x_i, &y_i)| y_i - get_function_value(self.function, &self.coefficients, x_i))
            .collect()
    }

    pub fn print_function(&self) -> String {
        match self.function {
            Function::Polynomial(m) => {
                let mut terms: Vec<String> = Vec::new();
                for i in 0..=m {
                    let coefficient = self.coefficients[i as usize];
                    // Skip adding term if coefficient is zero (except for the constant term if it's the only term)
                    if coefficient == 0.0 && i != 0 {
                        continue;
                    }
                    let mut term = if i == 0 || terms.is_empty() { // Check if terms is empty for the first non-zero coefficient
                        format!("{:.10}", coefficient)
                    } else {
                        format!("{:+.10}", coefficient)
                    };
    
                    if i == 1 {
                        term += "x"; // Handle exponent 1 without displaying ^1
                    } else if i > 1 {
                        term += &format!("x^{}", i);
                    }
                    terms.push(term);
                }
                if terms.is_empty() { // In case all coefficients are zero
                    terms.push("0".to_string());
                }
                terms.join("")
            }
            Function::Exponential => {
                format!(
                    "{:.10}e^{:+.10}x", // Remove plus for the first coefficient
                    self.coefficients[0], self.coefficients[1]
                )
            }
            Function::Logarithmic => {
                format!(
                    "{:.10} + {:+.10}\\ln(x)", // Remove plus for the first coefficient
                    self.coefficients[0], self.coefficients[1]
                )
            }
            Function::Power => {
                format!(
                    "{:.10}x^{:+.10}", // Remove plus for the first coefficient
                    self.coefficients[0], self.coefficients[1]
                )
            }
        }
    }
    
}

fn approximation_calculation(f: Function, n: usize, x: &Vec<f64>, y: &Vec<f64>) -> Vec<f64> {
    match f {
        Function::Polynomial(m) => {
            let mut b: Vec<f64> = vec![0.0; (m + 1) as usize];
            let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; (m + 1) as usize]; (m + 1) as usize];

            for i in 0..=m {
                for j in 0..n {
                    b[i as usize] += x[j].powi(i as i32) * y[j];
                }
            }

            for i in 0..=m {
                for j in 0..=m {
                    matrix[i as usize][j as usize] =
                        x.iter().map(|&v| v.powi((i + j) as i32)).sum();
                }
            }
            linear_calculation((m + 1) as usize, &mut matrix, &mut b, ACCURACY)
        }
        Function::Exponential => {
            let mut a = approximation_calculation(
                Function::Polynomial(1),
                n,
                x,
                &(y.iter().map(|&v| v.ln()).collect()),
            );
            a[0] = a[0].exp();
            a
        }
        Function::Logarithmic => approximation_calculation(
            Function::Polynomial(1),
            n,
            &(x.iter().map(|&v| v.ln()).collect()),
            y,
        ),
        Function::Power => approximation_calculation(
            Function::Polynomial(1),
            n,
            &(x.iter().map(|&v| v.ln()).collect()),
            &(y.iter().map(|&v| v.ln()).collect()),
        ),
    }
}

fn linear_calculation(n: usize, a: &mut [Vec<f64>], b: &mut [f64], e: f64) -> Vec<f64> {
    let mut v_x = vec![0.0; n];
    loop {
        let mut delta: f64 = 0.0;
        for i in 0..n {
            let mut s: f64 = 0.0;
            for j in 0..i {
                s += a[i][j] * v_x[j];
            }
            for j in i + 1..n {
                s += a[i][j] * v_x[j];
            }
            let x: f64 = (b[i] - s) / a[i][i];
            let d: f64 = (x - v_x[i]).abs();
            if d > delta {
                delta = d;
            }
            v_x[i] = x;
        }
        if delta < e {
            break;
        }
    }
    v_x
}

pub fn find_best_function(n: usize, x: &Vec<f64>, y: &Vec<f64>) -> Function {
    let mut deviations: Vec<(f64, Function)> = Vec::new();

    // Polynomial of degree 1 to 3
    for i in 1..=3 {
        let func = Function::Polynomial(i);
        let approximations = approximation_calculation(func, n, x, y);
        let differences = differences_calculation(func, n, &approximations, x, y);
        let deviation = standard_deviation_calculation(&differences, n);
        deviations.push((deviation, func));
    }

    // Exponential
    let exponential_approximations = approximation_calculation(Function::Exponential, n, x, y);
    let exponential_differences =
        differences_calculation(Function::Exponential, n, &exponential_approximations, x, y);
    deviations.push((
        standard_deviation_calculation(&exponential_differences, n),
        Function::Exponential,
    ));

    // Logarithmic
    let logarithmic_approximations = approximation_calculation(Function::Logarithmic, n, x, y);
    let logarithmic_differences =
        differences_calculation(Function::Logarithmic, n, &logarithmic_approximations, x, y);
    deviations.push((
        standard_deviation_calculation(&logarithmic_differences, n),
        Function::Logarithmic,
    ));

    // Power
    let power_approximations = approximation_calculation(Function::Power, n, x, y);
    let power_differences =
        differences_calculation(Function::Power, n, &power_approximations, x, y);
    deviations.push((
        standard_deviation_calculation(&power_differences, n),
        Function::Power,
    ));

    deviations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

    deviations[0].1
}

// Calculates the standard deviation of the differences
fn standard_deviation_calculation(differences: &[f64], n: usize) -> f64 {
    let sum: f64 = differences.iter().map(|&diff| diff.powi(2)).sum();
    let variance = sum / n as f64;
    variance.sqrt()
}

// Calculates the differences between the actual y values and those predicted by the function
fn differences_calculation(
    f: Function,
    n: usize,
    coefficients: &Vec<f64>,
    x: &[f64],
    y: &[f64],
) -> Vec<f64> {
    let mut differences = vec![0.0; n];
    for i in 0..n {
        differences[i] = y[i] - get_function_value(f, coefficients, x[i]);
    }
    differences
}

// Evaluates the given function at x using the provided coefficients
fn get_function_value(f: Function, coefficients: &[f64], x: f64) -> f64 {
    match f {
        Function::Polynomial(m) => {
            let mut sum = 0.0;
            for i in 0..=m {
                sum += coefficients[i as usize] * x.powi(i as i32);
            }
            sum
        }
        Function::Exponential => {
            // a[0] * exp(a[1] * x)
            coefficients[0] * (coefficients[1] * x).exp()
        }
        Function::Logarithmic => {
            // a[0] + a[1] * ln(x)
            coefficients[0] + coefficients[1] * x.ln()
        }
        Function::Power => {
            // a[0] * x^a[1]
            coefficients[0] * x.powf(coefficients[1])
        }
    }
}
