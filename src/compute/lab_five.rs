use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum InterpolationMethod {
    Lagrange,
    NewtonSeparated,
    NewtonFinite,
}

#[derive(Clone, Copy, Debug)]
pub enum StandardFunction {
    Sin,
    Cos,
    Tan,
}
pub struct InterpolationCalculator {
    method: InterpolationMethod,
    x: Vec<f64>,
    y: Vec<f64>,
}

impl InterpolationCalculator {
    pub fn new(method: InterpolationMethod, x: Vec<f64>, y: Vec<f64>) -> Self {
        if x.len() != y.len() {
            panic!("x and y must have the same length");
        }
        InterpolationCalculator { method, x, y }
    }

    pub fn interpolate(&self) -> Box<dyn Fn(f64) -> f64> {
        match self.method {
            InterpolationMethod::Lagrange => self.lagrange(),
            InterpolationMethod::NewtonSeparated => self.newton(),
            _ => panic!("Invalid interpolation method"),
        }
    }

    fn lagrange(&self) -> Box<dyn Fn(f64) -> f64> {
        let x = self.x.clone();
        let y = self.y.clone();
        Box::new(move |v: f64| {
            let mut sum = 0.0;
            for i in 0..x.len() {
                let mut product = 1.0;
                for j in 0..x.len() {
                    if i != j {
                        product *= (v - x[j]) / (x[i] - x[j]);
                    }
                }
                sum += y[i] * product;
            }
            sum
        })
    }

    fn newton(&self) -> Box<dyn Fn(f64) -> f64> {
        let diff = self.differences();
        let x = self.x.clone();
        Box::new(move |v: f64| {
            let mut sum = diff[0];
            for i in 1..x.len() {
                let mut product = 1.0;
                for j in 0..i {
                    product *= v - x[j];
                }
                sum += diff[i] * product;
            }
            sum
        })
    }

    fn differences(&self) -> Vec<f64> {
        let mut diff = self.y.clone();
        for i in 1..self.y.len() {
            for j in (i..self.y.len()).rev() {
                diff[j] = (diff[j] - diff[j-1]) / (self.x[j] - self.x[j-i]);
            }
        }
        diff
    }

    pub fn print_latex(&self) -> String {
        match self.method {
            InterpolationMethod::Lagrange => self.lagrange_latex(),
            InterpolationMethod::NewtonSeparated => self.newton_latex(),
            _ => panic!("Invalid interpolation method"),
        }
    }

    fn lagrange_latex(&self) -> String {
        let mut terms = vec![];
        for i in 0..self.x.len() {
            let mut term_parts = vec![];
            for j in 0..self.x.len() {
                if i != j {
                    term_parts.push(format!("(x - {:.3})", self.x[j]));
                }
            }
            let term = if !term_parts.is_empty() {
                format!("{} \\cdot {}", self.y[i], term_parts.join(" \\cdot "))
            } else {
                format!("{}", self.y[i])
            };
            terms.push(term);
        }
        format!("P(x) = {}", terms.join(" + "))
    }

    fn newton_latex(&self) -> String {
        let diff = self.differences();
        let mut terms = vec![format!("{}", diff[0])];
        for i in 1..self.x.len() {
            let mut term_parts = vec![];
            for j in 0..i {
                term_parts.push(format!("(x - {:.3})", self.x[j]));
            }
            let term = format!("{} \\cdot {}", diff[i], term_parts.join(" \\cdot "));
            terms.push(term);
        }
        format!("P(x) = {}", terms.join(" + "))
    }
}

pub fn generate_function_values(func: fn(f64) -> f64, start: f64, end: f64, nodes_amount: usize) -> (Vec<f64>, Vec<f64>) {
    let step = (end - start) / (nodes_amount as f64 - 1.0);
    let mut x = Vec::with_capacity(nodes_amount);
    let mut y = Vec::with_capacity(nodes_amount);
    for i in 0..nodes_amount {
        let x_value = start + step * i as f64;
        x.push(x_value);
        y.push(func(x_value));
    }
    (x, y)
}