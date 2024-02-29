use graphul::{
    extract::Json,
    http::{utils::header::CONTENT_TYPE, Methods},
    Context, Graphul,
};

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
}

pub struct Solver<'a> {
    equation: &'a Equation,
    method: MethodType,
    n: u32,
}

impl Equation {
    pub fn new(number: u8) -> Self {
        match number {
            1 => Self::Equation1,
            2 => Self::Equation2,
            3 => Self::Equation3,
            4 => Self::Equation4,
            _ => panic!("Invalid equation number"),
        }
    }

    fn get_value(&self, x: f64) -> f64 {
        match self {
            Self::Equation1 => 1.62 * x.powi(3) - 8.15 * x.powi(2) + 4.39 * x + 4.29,
            Self::Equation2 => x.powi(3) - x + 4.0,
            Self::Equation3 => x.exp() - 5.0,
            Self::Equation4 => (2.0 * x).sin() + PI / 4.0,
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
    fn new_function(&self, x: f64, parameter_lambda: f64) -> f64 {
        x + parameter_lambda * self.get_value(x)
    }

    fn new_function_first_derivative(&self, x: f64, parameter_lambda: f64) -> f64 {
        1.0 + parameter_lambda * self.derivative(x, 1)
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
        }
    }

    fn solve_half_division(
        &mut self,
        mut left: f64,
        mut right: f64,
        estimate: f64,
    ) -> Json<serde_json::Value> {
        if self.equation.get_value(left) * self.equation.get_value(right) >= 0.0 {
            return Json(json!({"error": "No root found in the given interval."}));
        }

        if self.equation.get_value(left) * self.equation.get_value(right) > 0.0 {
            return Json(json!({
                "error": "На данном участке нет корней."
            }));
        }

        let mut steps = Vec::new();
        let mut x = (left + right) / 2.0;
        steps.push(self.create_step(left, right, x));

        while (right - left).abs() > estimate {
            self.n += 1;
            let f_left = self.equation.get_value(left);
            let fx = self.equation.get_value(x);

            if f_left * fx > 0.0 {
                left = x;
            } else {
                right = x;
            }
            x = (left + right) / 2.0;
            steps.push(self.create_step(left, right, x));
        }

        let result = json!({
            "result": {
                "root": x,
                "function_value": self.equation.get_value(x),
                "iterations": self.n + 1,
                "steps": steps,
                "err": ""
            }
        });
        Json(result)
    }

    fn solve_iteration(&mut self, left: f64, right: f64, estimate: f64) -> Json<serde_json::Value> {
        if self.equation.get_value(left) * self.equation.get_value(right) > 0.0 {
            return Json(json!({"error": "На данном участке нет корней."}));
        }

        let parameter_lambda = -1.0
            / self
                .equation
                .derivative(left, 1)
                .max(self.equation.derivative(right, 1));
        if self
            .equation
            .new_function_first_derivative(left, parameter_lambda)
            .abs()
            >= 1.0
            || self
                .equation
                .new_function_first_derivative(right, parameter_lambda)
                .abs()
                >= 1.0
        {
            return Json(json!({
                "error": "Метод не сходится. Нарушено достаточное условие сходимости метода.",
                "phi_prime_a": self.equation.new_function_first_derivative(left, parameter_lambda),
                "phi_prime_b": self.equation.new_function_first_derivative(right, parameter_lambda),
            }));
        }

        let x0 = if self.equation.get_value(right) * self.equation.derivative(right, 2) > 0.0 {
            right
        } else {
            left
        };

        let mut x = x0;
        let mut steps = Vec::new();

        loop {
            self.n += 1;
            let x_next = self.equation.new_function(x, parameter_lambda);
            steps.push(json!({
                "iteration": self.n,
                "x_k": x,
                "f_x_k": self.equation.get_value(x),
                "x_k_plus_one": x_next,
                "phi_x_k": self.equation.new_function(x, parameter_lambda),
                "abs_diff": (x - x_next).abs(),
                "err": ""
            }));

            if (x - x_next).abs() <= estimate {
                break;
            }

            x = x_next;
        }

        Json(json!({
            "result": {
                "root": x,
                "function_value": self.equation.get_value(x),
                "iterations": self.n,
                "steps": steps,
            }
        }))
    }

    fn solve_newton(&mut self, left: f64, right: f64, estimate: f64) -> Json<serde_json::Value> {
        if self.equation.get_value(left) * self.equation.get_value(right) > 0.0 {
            return Json(json!({"error": "На данном участке нет корней."}));
        }

        let mut x0 = if self.equation.get_value(right) * self.equation.derivative(right, 1) > 0.0 {
            right
        } else {
            left
        };

        let mut steps = Vec::new();
        let mut x;

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
                "root": x,
                "function_value": self.equation.get_value(x),
                "iterations": self.n,
                "steps": steps,
            }
        }))
    }

    fn create_step(&self, a: f64, b: f64, x: f64) -> Value {
        json!({
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
