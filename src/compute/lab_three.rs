use serde_json::json;
use serde_json::Value;
use std::f64::consts::PI;

pub enum Function {
    Polynomial,
    Sinus,
    Linear,
    Exponential,
}

pub enum IntegrationMethod {
    LeftRectangles,
    RightRectangles,
    MiddleRectangles,
    Trapezoid,
    Simpson,
}

pub struct IntegralCalculator {
    function: Function,
    method: IntegrationMethod,
    error: f64,
    lower_bound: f64,
    upper_bound: f64,
}

impl Function {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Self::Polynomial => 2.0 * x.powi(3) - 9.0 * x.powi(2) - 7.0 * x + 11.0,
            Self::Sinus => (x * PI / 180.0).sin(),
            Self::Linear => 2.0 * x,
            Self::Exponential => 1.0 / x,
        }
    }
}

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
        match self.integrate() {
            Some((result, iterations)) => json!({
                "calculated_integral": result,
                "iterations": iterations,
            }),
            None => json!({
                "error": "Интеграл не существует"
            }),
        }
    }

    fn integrate(&self) -> Option<(f64, i32)> {
        let f = |x: f64| -> f64 { self.function.evaluate(x) };

        // Проверка на существование интеграла
        if Self::integration_limit_at_a(&f, self.lower_bound, self.upper_bound).is_none()
            || Self::integration_limit_at_b(&f, self.lower_bound, self.upper_bound).is_none()
            || Self::integration_limit_on_interval(&f, self.lower_bound, self.upper_bound).is_none()
        {
            return None;
        }

        // Если интеграл существует, вычислить его
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

        Some((result, n))
    }

    fn integration_limit_at_a(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> Option<f64> {
        let mut sum = 0.0;
        let mut step = 1.0;
        let mut prev_value = f(a + step);

        while a + step < b {
            let curr_value = f(a + step);
            if curr_value.is_infinite() {
                return None; // Интеграл не существует
            }
            sum += prev_value;
            prev_value = curr_value;
            step *= 2.0;
        }

        Some(sum)
    }

    fn integration_limit_at_b(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> Option<f64> {
        let mut sum = 0.0;
        let mut step = 1.0;
        let mut prev_value = f(b - step);

        while b - step > a {
            let curr_value = f(b - step);
            if curr_value.is_infinite() {
                return None; // Интеграл не существует
            }
            sum += prev_value;
            prev_value = curr_value;
            step *= 2.0;
        }

        Some(sum)
    }

    fn integration_limit_on_interval(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> Option<f64> {
        let mut sum = 0.0;
        let mut step = 1.0;
        let mut prev_value = f(a + step);

        while a + step < b {
            let curr_value = f(a + step);
            if curr_value.is_infinite() {
                return None; // Интеграл не существует
            }
            sum += prev_value;
            prev_value = curr_value;
            step *= 2.0;
        }

        Some(sum)
    }

    fn apply_method(&self, n: i32, f: &dyn Fn(f64) -> f64) -> f64 {
        let (a, b) = (self.lower_bound, self.upper_bound);
        let h = (b - a) / n as f64;
        match self.method {
            IntegrationMethod::LeftRectangles => {
                (0..n).map(|i| f(a + i as f64 * h)).sum::<f64>() * h
            }
            IntegrationMethod::RightRectangles => {
                (1..=n).map(|i| f(a + i as f64 * h)).sum::<f64>() * h
            }
            IntegrationMethod::MiddleRectangles => {
                (0..n).map(|i| f(a + (i as f64 + 0.5) * h)).sum::<f64>() * h
            }
            IntegrationMethod::Trapezoid => {
                0.5 * h * (f(a) + f(b) + 2.0 * (1..n).map(|i| f(a + i as f64 * h)).sum::<f64>())
            }
            IntegrationMethod::Simpson => {
                h / 3.0
                    * (f(a)
                        + f(b)
                        + 4.0 * (1..n).step_by(2).map(|i| f(a + i as f64 * h)).sum::<f64>()
                        + 2.0
                            * (2..n - 1)
                                .step_by(2)
                                .map(|i| f(a + i as f64 * h))
                                .sum::<f64>())
            }
        }
    }
}
