use serde_json::json;
use serde_json::Value;
use std::f64::consts::PI;

// Define the available functions as an enum
pub enum Function {
    Polynomial,
    Sinus,
    Linear,
    Exponential,
}

// Define the available methods as an enum
pub enum IntegrationMethod {
    LeftRectangles,
    RightRectangles,
    MiddleRectangles,
    Trapezoid,
    Simpson,
}

// A struct to encapsulate the calculation parameters and methods
pub struct IntegralCalculator {
    function: Function,
    method: IntegrationMethod,
    error: f64,
    lower_bound: f64,
    upper_bound: f64,
}

// Implementation of function behaviors
impl Function {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Self::Polynomial => 2.0 * x.powi(3) - 9.0 * x.powi(2) - 7.0 * x + 11.0,
            Self::Sinus => (x * PI / 180.0).sin(),
            Self::Linear => 2.0 * x,
            Self::Exponential => x.exp(),
        }
    }
}

// Implementation of IntegralCalculator
impl IntegralCalculator {
    pub fn new(
        function: Function,
        method: IntegrationMethod,
        error: f64,
        lower_bound: f64,
        upper_bound: f64,
    ) -> Self {
        Self {
            function,
            method,
            error,
            lower_bound,
            upper_bound,
        }
    }

    pub fn calculate_integral(&self) -> Value {
        let (result, iterations) = self.integrate();
        json!({
            "Calculated Integral": result,
            "Number of Iterations": iterations,
        })
    }

    fn integrate(&self) -> (f64, i32) {
        let f = |x: f64| -> f64 { self.function.evaluate(x) };
        let (mut result, mut n) = (0.0, 1);
        let mut i_0 = self.apply_method(n, &f);
        let error_target = self.error;

        loop {
            n *= 2;
            let i_1 = self.apply_method(n, &f);
            if ((i_0 - i_1) / error_target).abs() < self.error {
                result = i_1;
                break;
            }
            i_0 = i_1;
        }

        (result, n)
    }

    fn apply_method(&self, n: i32, f: &dyn Fn(f64) -> f64) -> f64 {
        let (a, b) = (self.lower_bound, self.upper_bound);
        let h = (b - a) / n as f64;
        match self.method {
            IntegrationMethod::LeftRectangles => (0..n).map(|i| f(a + i as f64 * h)).sum::<f64>() * h,
            IntegrationMethod::RightRectangles => (1..=n).map(|i| f(a + i as f64 * h)).sum::<f64>() * h,
            IntegrationMethod::MiddleRectangles => (0..n).map(|i| f(a + (i as f64 + 0.5) * h)).sum::<f64>() * h,
            IntegrationMethod::Trapezoid => {
                0.5 * h * (f(a) + f(b) + 2.0 * (1..n).map(|i| f(a + i as f64 * h)).sum::<f64>())
            }
            IntegrationMethod::Simpson => {
                h / 3.0 * (f(a) + f(b) + 4.0 * (1..n).step_by(2).map(|i| f(a + i as f64 * h)).sum::<f64>() + 2.0 * (2..n-1).step_by(2).map(|i| f(a + i as f64 * h)).sum::<f64>())
            }
        }
    }
}

