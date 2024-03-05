use graphul::extract::Json;

use serde_json::{json, Value};
use std::f64::consts::PI;

pub enum Equation {
    Equation1,
    Equation2,
    Equation3,
    Equation4,
}

pub enum MethodType {
    HalfDivision,
    Iteration,
    Newton,
    Secant,
}

pub struct Solver<'a> {
    equation: &'a Equation,
    method: MethodType,
    n: u32,
}

impl Equation {
    pub fn new(number: u8) -> Self {
        match number {
            0 => Self::Equation1,
            1 => Self::Equation2,
            2 => Self::Equation3,
            3 => Self::Equation4,
            _ => panic!("Invalid equation number"),
        }
    }

    fn get_value(&self, x: f64) -> f64 {
        match self {
            Self::Equation1 => 1.62 * x.powi(3) - 8.15 * x.powi(2) + 4.39 * x + 4.29,
            // Self::Equation1 => 2.0 * x.powi(3) - 9.0 * x.powi(2) - 7.0 * x + 11.0,
            Self::Equation2 => x.powi(3) - x + 4.0,
            // Self::Equation2 => -1.8*x.powi(3)-2.94*x.powi(2)+10.37*x+5.38,
            Self::Equation3 => x.exp() - 5.0,
            Self::Equation4 => (2.0 * x).sin() + PI / 4.0,
        }
    }

    fn get_function_index(&self) -> usize {
        match self {
            Self::Equation1 => 1 - 1,
            Self::Equation2 => 2 - 1,
            Self::Equation3 => 3 - 1,
            Self::Equation4 => 4 - 1,
        }
    }

    fn derivative(&self, x: f64, order: u8) -> f64 {
        let h = 0.00001;
        match order {
            1 => (self.get_value(x + h) - self.get_value(x)) / h,
            2 => {
                (self.get_value(x + h) - 2.0 * self.get_value(x) + self.get_value(x - h)) / (h * h)
            }
            _ => panic!("Unsupported derivative order"),
        }
    }
}

impl<'a> Solver<'a> {
    pub fn new(eq: &'a Equation, method: MethodType) -> Self {
        Self {
            equation: eq,
            method,
            n: 0,
        }
    }

    pub fn solve(&mut self, left: f64, right: f64, estimate: f64) -> Json<serde_json::Value> {
        match self.method {
            MethodType::HalfDivision => self.solve_half_division(left, right, estimate),
            MethodType::Iteration => self.solve_iteration(left, right, estimate),
            MethodType::Newton => self.solve_newton(left, right, estimate),
            MethodType::Secant => {
                let x0 = left;
                let x1 = right;
                self.solve_secant(x0, x1, estimate)
            }
        }
    }

    fn solve_half_division(
        &mut self,
        mut left: f64,
        mut right: f64,
        estimate: f64,
    ) -> Json<serde_json::Value> {
        if self.equation.get_value(left) * self.equation.get_value(right) >= 0.0 {
            return Json(
                json!({"error": "Function values at the interval endpoints must have opposite signs"}),
            );
        }

        let mut steps = Vec::new();
        let mut x;
        let og_left = left;
        let og_right = right;
        let og_estimate = estimate;

        loop {
            x = (left + right) / 2.0;

            steps.push(self.create_step(left, right, x));

            if self.equation.get_value(left) * self.equation.get_value(x) > 0.0 {
                left = x;
            } else {
                right = x;
            }

            if (right - left).abs() < estimate {
                let n = (estimate.log10().abs().ceil() as u32).max(1); // Ensure n is at least 1
                let multiplier = 10f64.powi(n as i32);
                let result_x = (x * multiplier).ceil() / multiplier;
                let result_fx = (self.equation.get_value(x) * multiplier).ceil() / multiplier;

                self.n += 1;

                return Json(json!({
                    "result": {
                        "left": og_left,
                        "right": og_right,
                        "error_value": (right - left).abs(),
                        "estimate": og_estimate,
                        "eq_id": self.equation.get_function_index(),
                        "method_id": 0,
                        "root": result_x,
                        "function_value": result_fx,
                        "iterations": self.n,
                        "steps": steps,
                        "err": ""
                    }
                }));
            }
            self.n += 1;
        }
    }

    fn solve_iteration(&mut self, left: f64, right: f64, estimate: f64) -> Json<serde_json::Value> {
        if self.equation.get_value(left) * self.equation.get_value(right) >= 0.0 {
            return Json(
                json!({"error": "Function values at the interval endpoints must have opposite signs"}),
            );
        }
        let og_left = left;
        let og_right = right;
        let og_estimate = estimate;
        let sigma = self
            .equation
            .derivative(left, 1)
            .abs()
            .max(self.equation.derivative(right, 1).abs());
        let x0 =
            if self.equation.derivative(left, 1).abs() > self.equation.derivative(right, 1).abs() {
                left
            } else {
                right
            };

        // Check for convergence condition
        if 1.0 - self.equation.derivative(left, 1) / sigma >= 1.0
            && 1.0 - self.equation.derivative(right, 1) / sigma >= 1.0
        {
            return Json(json!({"error": "Method does not converge"}));
        }

        let mut x = x0;
        let mut res = std::f64::INFINITY; // Initialize `res` with infinity for the first iteration
        let mut steps = Vec::new();

        loop {
            let x_next = x - self.equation.get_value(x) / sigma;

            if (x0 - x_next).abs() > res {
                // If the distance between the new and old values is greater than the previous max distance,
                // it indicates the method may not converge in the current interval.
                return Json(
                    json!({"error": "Narrow down the interval. The method converges only in a small neighborhood of the root."}),
                );
            }

            steps.push(json!({
                "key": self.n,
                "iteration": self.n,
                "x_k": x,
                "x_k_plus_one": x_next,
                "f_x_k": self.equation.get_value(x_next),
                "abs_diff": (x - x_next).abs(),
                "err": ""
            }));

            if (x_next - x).abs() < estimate {
                let n = estimate.log10().abs().ceil() as u32;
                let multiplier = 10f64.powi(n as i32);
                let result_x = (x_next * multiplier).ceil() / multiplier;
                let result_fx = (self.equation.get_value(x_next) * multiplier).ceil() / multiplier;

                self.n += 1;

                return Json(json!({
                    "result": {
                        "error_value": (x_next - x).abs(),
                        "left": og_left,
                        "right": og_right,
                        "estimate": og_estimate,
                        "eq_id": self.equation.get_function_index(),
                        "method_id": 1,
                        "root": result_x,
                        "function_value": result_fx,
                        "iterations": self.n,
                        "steps": steps,
                    }
                }));
            }

            res = (x_next - x).abs();
            x = x_next;
            self.n += 1;
        }
    }

    fn solve_newton(&mut self, left: f64, right: f64, estimate: f64) -> Json<serde_json::Value> {
        if self.equation.get_value(left) * self.equation.get_value(right) >= 0.0 {
            return Json(
                json!({"error": "Function values at the interval endpoints must have opposite signs"}),
            );
        }

        let og_estimate = estimate;
        let og_left = left;
        let og_right = right;
        let mut steps = Vec::new();
        let mut x;

        let mut x0 = if self.equation.get_value(right) * self.equation.derivative(right, 1) > 0.0 {
            right
        } else {
            left
        };

        loop {
            self.n += 1;
            let f_x0 = self.equation.get_value(x0);
            let df_x0 = self.equation.derivative(x0, 1);
            if df_x0 == 0.0 {
                return Json(
                    json!({"error": "Уточните входной интервал. Первая производная на промежутке равна нулю"}),
                );
            }

            x = x0 - (f_x0 / df_x0);

            steps.push(json!({
                "key": self.n,
                "iteration": self.n,
                "x_k": x0,
                "f_x_k": f_x0,
                "f_prime_x_k": df_x0,
                "x_k_plus_one": x,
                "abs_diff": (x - x0).abs(),
            }));

            if (x - x0).abs() <= estimate && self.equation.get_value(x).abs() < estimate {
                break;
            }

            x0 = x;
        }

        Json(json!({
            "result": {
                "error_value": (x - x0).abs(),
                "left": og_left,
                "right": og_right,
                "estimate": og_estimate,
                "eq_id": self.equation.get_function_index(),
                "method_id": 2,
                "root": x,
                "function_value": self.equation.get_value(x),
                "iterations": self.n,
                "steps": steps,
            }
        }))
    }

    fn solve_secant(&mut self, mut x0: f64, mut x1: f64, estimate: f64) -> Json<serde_json::Value> {
        let mut steps = Vec::new();
        let og_left = x0;
        let og_right = x1;
        let og_estimate = estimate;

        let denominator = self.equation.get_value(x1) - self.equation.get_value(x0);
        if denominator.abs() < std::f64::EPSILON {
            return Json(json!({"error": "Denominator too small, secant method cannot proceed"}));
        }

        loop {
            let x2 = x1
                - self.equation.get_value(x1) * (x1 - x0)
                    / (self.equation.get_value(x1) - self.equation.get_value(x0));
            let abs_diff = (x2 - x1).abs();

            steps.push(json!({
                "key": self.n,
                "iteration": self.n,
                "x_k_1": x0,
                "x_k": x1,
                "x_k_plus_one": x2,
                "f_x_k_plus_one": self.equation.get_value(x2),
                "abs_diff": abs_diff,
            }));

            if abs_diff < estimate {
                let n = estimate.log10().abs().ceil() as u32;
                let multiplier = 10f64.powi(n as i32);
                let result_x = (x2 * multiplier).ceil() / multiplier;
                let result_fx = (self.equation.get_value(x2) * multiplier).ceil() / multiplier;

                self.n += 1; // Increment the step counter

                return Json(json!({
                    "result": {
                        "error_value": abs_diff,
                        "left": og_left,
                        "right": og_right,
                        "estimate": og_estimate,
                        "eq_id": self.equation.get_function_index(),
                        "method_id": 3,
                        "root": result_x,
                        "function_value": result_fx,
                        "iterations": self.n,
                        "steps": steps,
                    }
                }));
            }

            x0 = x1;
            x1 = x2;
            self.n += 1;
        }
    }

    fn create_step(&self, a: f64, b: f64, x: f64) -> Value {
        json!({
            "key": self.n,
            "iteration": self.n,
            "a": a,
            "b": b,
            "x": x,
            "fa": self.equation.get_value(a),
            "fb": self.equation.get_value(b),
            "fx": self.equation.get_value(x),
            "abs_diff": (b - a).abs(),
            "err": ""
        })
    }
}

pub enum SystemEquations {
    EquationSystem1,
    EquationSystem2,
    EquationSystem3,
}

pub struct NewtonSystemMethod<'a> {
    x0: f64,
    y0: f64,
    tolerance: f64,
    equations: &'a SystemEquations,
    counter: usize,
}

impl SystemEquations {
    pub fn new(number: u8) -> Self {
        match number {
            0 => Self::EquationSystem1,
            1 => Self::EquationSystem2,
            2 => Self::EquationSystem3,
            _ => panic!("Invalid equation number"),
        }
    }
    fn get_value(&self, x: f64, y: f64) -> (f64, f64) {
        match self {
            SystemEquations::EquationSystem1 => (x.powi(2) + y.powi(2) - 4.0, -3.0 * x.powi(2) + y),
            SystemEquations::EquationSystem2 => (
                x.powi(2) + x - y.powi(2) - 0.15,
                x.powi(2) - y + y.powi(2) + 0.17,
            ),
            SystemEquations::EquationSystem3 => (2.0 * y - (x + 1.0).cos(), x + y.sin() + 0.4),
        }
    }
    fn get_function_index(&self) -> usize {
        match self {
            Self::EquationSystem1 => 1 - 1,
            Self::EquationSystem2 => 2 - 1,
            Self::EquationSystem3 => 3 - 1,
        }
    }
}

impl<'a> NewtonSystemMethod<'a> {
    pub fn new(x0: f64, y0: f64, tolerance: f64, equations: &'a SystemEquations) -> Self {
        Self {
            x0,
            y0,
            tolerance,
            equations,
            counter: 0,
        }
    }

    fn partial_derivatives(&self) -> ((f64, f64), (f64, f64)) {
        let h = 0.0001;
        let fx0_y0 = self.equations.get_value(self.x0, self.y0).0;
        let fy0_y0 = self.equations.get_value(self.x0, self.y0).1;

        let fx_h_y0 = self.equations.get_value(self.x0 + h, self.y0).0;
        let fy_h_y0 = self.equations.get_value(self.x0 + h, self.y0).1;

        let fx0_h_y0 = self.equations.get_value(self.x0, self.y0 + h).0;
        let fy0_h_y0 = self.equations.get_value(self.x0, self.y0 + h).1;

        let dxf = (fx_h_y0 - fx0_y0) / h;
        let dyf = (fx0_h_y0 - fx0_y0) / h;

        let dxg = (fy_h_y0 - fy0_y0) / h;
        let dyg = (fy0_h_y0 - fy0_y0) / h;

        ((dxf, dyf), (dxg, dyg))
    }

    pub fn solve(&mut self) -> Value {
        let ((dxf, dyf), (dxg, dyg)) = self.partial_derivatives();

        // Calculate the Jacobian determinant
        let jacobian_determinant = dxf * dyg - dyf * dxg;

        if jacobian_determinant == 0.0 {
            return json!({"error": "Jacobian determinant is zero, system does not meet the sufficient condition for convergence"});
        }

        let (fx, fy) = self.equations.get_value(self.x0, self.y0);

        let a = [[dxf, dyf], [dxg, dyg]];
        let b = [-fx, -fy];
        let roots = Self::solve_kramer(a, b);

        let x1 = self.x0 + roots[0];
        let y1 = self.y0 + roots[1];
        let estimate = ((x1 - self.x0).powi(2) + (y1 - self.y0).powi(2)).sqrt();

        if estimate < self.tolerance {
            json!({
                "result":{
                    "eq_id": self.equations.get_function_index(),
                    "x": x1,
                    "y": y1,
                    "iterations": self.counter,
                    "error_value": estimate,
                }
            })
        } else {
            self.x0 = x1;
            self.y0 = y1;
            self.counter += 1;
            self.solve()
        }
    }

    pub fn solve_kramer(a: [[f64; 2]; 2], b: [f64; 2]) -> [f64; 2] {
        let d = a[0][0] * a[1][1] - a[1][0] * a[0][1];
        let dx = b[0] * a[1][1] - b[1] * a[0][1];
        let dy = a[0][0] * b[1] - a[1][0] * b[0];
        if d == 0.0 {
            panic!("Determinant is zero, system has no unique solution");
        }
        [dx / d, dy / d]
    }
}
