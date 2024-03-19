use serde_json::json;
use serde_json::Value;
use std::f64::consts::E;
use std::f64::{INFINITY, NEG_INFINITY};

pub enum Function {
    Polynomial,
    Sinus,
    Linear,
    ScaledLogistic,
    Hyperbola,
    SqrtHyperbola,
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

struct Interval {
    ranges: Vec<(f64, f64)>,
    points: Vec<f64>,
}

impl Interval {
    fn new(ranges: Vec<(f64, f64)>, points: Vec<f64>) -> Self {
        Interval { ranges, points }
    }
}

impl Function {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Self::Polynomial => x.powi(3) - 3.0 * x.powi(2) + 7.0 * x - 10.0,
            Self::Sinus => x.sin(),
            Self::Linear => x,
            Self::ScaledLogistic => x / (1.0 + x.powi(2)).sqrt(),
            Self::Hyperbola => 1.0 / x,
            Self::SqrtHyperbola => 1.0 / x.sqrt(),
        }
    }

    fn evaluate_antiderivative(&self, x: f64) -> f64 {
        match self {
            Self::Polynomial => 0.25 * x.powi(4) - x.powi(3) + 3.5 * x.powi(2) - 10.0 * x,
            Self::Sinus => -x.cos(),
            Self::Linear => 0.5 * x.powi(2),
            Self::ScaledLogistic => (1.0 + x.powi(2)).sqrt(),
            Self::Hyperbola => x.abs().log(E),
            Self::SqrtHyperbola => 2.0 * x.sqrt(),
        }
    }

    fn get_points_of_infinite_discontinuity(&self) -> Interval {
        match self {
            Self::Polynomial | Self::Sinus | Self::Linear | Self::ScaledLogistic => {
                Interval::new(vec![(NEG_INFINITY, INFINITY)], vec![])
            }
            Self::Hyperbola => Interval::new(vec![(NEG_INFINITY, 0.0), (0.0, INFINITY)], vec![0.0]),
            Self::SqrtHyperbola => Interval::new(vec![(0.0, INFINITY)], vec![0.0]),
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
    fn adjust_intervals_for_symmetry(&self, a: f64, b: f64) -> Vec<(f64, f64)> {
        let interval_data = self.function.get_points_of_infinite_discontinuity();
        let mut adjusted_intervals = vec![];

        for point in interval_data.points {
            // Check if the point of discontinuity is within the bounds
            if point >= a && point <= b {
                let left = self.function.evaluate(point - 0.0001);
                let right = self.function.evaluate(point + 0.0001);

                // Check for symmetry and opposite signs
                if left.is_sign_negative() != right.is_sign_negative() {
                    // If the function is symmetric around the point and has opposite signs,
                    // we can ignore the symmetrical interval around this point of discontinuity.
                    if a < point {
                        adjusted_intervals.push((a, point - 0.0001));
                    }
                    if b > point {
                        adjusted_intervals.push((point + 0.0001, b));
                    }
                    continue;
                }
            }
            // If no special handling is needed, add the original interval
            adjusted_intervals.push((a, b));
        }

        // If no adjustments were made, return the original interval
        if adjusted_intervals.is_empty() {
            adjusted_intervals.push((a, b));
        }

        adjusted_intervals
    }

    pub fn calculate_integral(&self) -> Value {
        // Adjust intervals based on symmetry and opposite signs around discontinuities
        let adjusted_intervals = self.adjust_intervals_for_symmetry(self.lower_bound, self.upper_bound);

        // Calculate integral for each adjusted interval
        let mut results = vec![];
        for (start, end) in adjusted_intervals {
            let result = self.check_and_calculate_integral(start, end);
            results.push(result);
        }

        // Aggregate results or handle error cases
        if results.is_empty() {
            json!({"error": "Could not calculate the integral."})
        } else if results.len() == 1 {
            results[0].clone()
        } else {
            json!({"intervals": results})
        }
    }

    fn check_and_calculate_integral(&self, a: f64, b: f64) -> Value {
        if !self.check_domain(a, b) {
            return json!({"error": "Interval is not within the domain of the function."});
        }

        let convergence = self.check_convergence(a, b);
        if convergence.is_empty() {
            return json!({"error": "The function does not converge in the given interval."});
        }

        let results: Vec<Value> = convergence
            .into_iter()
            .filter_map(|(start, end)| {
                self.calculate_integral_specific_range(start, end).map(
                    |(integral, subdivisions)| // Note: "subdivisions" is equal to the number of iterations
                    json!({
                        "interval": {"start": start, "end": end},
                        "integral_value": integral,
                        "iterations": subdivisions, // Using subdivisions as iterations count
                    }),
                )
            })
            .collect();

        if results.is_empty() {
            json!({"error": "Could not calculate the integral."})
        } else {
            json!(results[0])
        }
    }

    fn check_domain(&self, a: f64, b: f64) -> bool {
        let interval_data = self.function.get_points_of_infinite_discontinuity();
        !interval_data.ranges.is_empty()
            && interval_data
                .ranges
                .iter()
                .any(|&(start, end)| start <= a && end >= b)
    }

    fn check_convergence(&self, a: f64, b: f64) -> Vec<(f64, f64)> {
        let mut intervals = Vec::new();
        let interval_data = self.function.get_points_of_infinite_discontinuity();
        let points = interval_data.points;
        let mut intervals_count = 0usize;

        for &point in &points {
            if point < a || point > b {
                continue;
            }
            let antiderivative_value = self.function.evaluate_antiderivative(point);

            if antiderivative_value.is_nan() || antiderivative_value.is_infinite() {
                return Vec::new(); // Function does not converge in this interval
            }

            let deviation = 0.000000001f64;
            if (point - a).abs() < deviation {
                intervals.push((point + deviation, b));
                intervals_count += 1;
            } else if (b - point).abs() < deviation {
                if intervals.is_empty() {
                    intervals.push((a, b - deviation));
                } else {
                    intervals[intervals_count - 1].1 = b - deviation;
                }
            } else {
                if intervals_count > 0 {
                    intervals[intervals_count - 1].1 = point - deviation;
                }
                intervals.push((point + deviation, b));
                intervals_count += 1;
            }
        }

        if intervals.is_empty() {
            vec![(a, b)] // If no adjustments were made, return the original interval
        } else {
            intervals
        }
    }

    fn calculate_integral_specific_range(&self, start: f64, end: f64) -> Option<(f64, i32)> {
        let mut result = 0.0;
        let mut n = 1; // Initial number of subdivisions
        let mut i_h = self.apply_method_specific_range(n, start, end); // Initial approximation
        let mut error = self.error + 1.0; // Initial error

        while error > self.error {
            n *= 2; // Double the number of subdivisions for each iteration
            let i_h2 = self.apply_method_specific_range(n, start, end); // New approximation
            error = match self.method {
                IntegrationMethod::Simpson => ((i_h2 - i_h) / 15.0).abs(),
                _ => ((i_h2 - i_h) / 3.0).abs(),
            };

            if error < self.error {
                result = i_h2; // Update result with the new approximation
                break;
            }

            i_h = i_h2; // Update for next iteration
        }

        Some((result, n)) // Return both result and number of subdivisions (iterations)
    }

    fn apply_method_specific_range(&self, n: i32, start: f64, end: f64) -> f64 {
        let h = (end - start) / n as f64;
        match self.method {
            IntegrationMethod::LeftRectangles => {
                (0..n)
                    .map(|i| self.function.evaluate(start + i as f64 * h))
                    .sum::<f64>()
                    * h
            }
            IntegrationMethod::RightRectangles => {
                (1..=n)
                    .map(|i| self.function.evaluate(start + i as f64 * h))
                    .sum::<f64>()
                    * h
            }
            IntegrationMethod::MiddleRectangles => {
                (0..n)
                    .map(|i| self.function.evaluate(start + (i as f64 + 0.5) * h))
                    .sum::<f64>()
                    * h
            }
            IntegrationMethod::Trapezoid => {
                0.5 * h
                    * (self.function.evaluate(start)
                        + self.function.evaluate(end)
                        + 2.0
                            * (1..n)
                                .map(|i| self.function.evaluate(start + i as f64 * h))
                                .sum::<f64>())
            }
            IntegrationMethod::Simpson => {
                h / 3.0
                    * (self.function.evaluate(start)
                        + self.function.evaluate(end)
                        + 4.0
                            * (1..n)
                                .step_by(2)
                                .map(|i| self.function.evaluate(start + i as f64 * h))
                                .sum::<f64>()
                        + 2.0
                            * (2..n - 1)
                                .step_by(2)
                                .map(|i| self.function.evaluate(start + i as f64 * h))
                                .sum::<f64>())
            }
        }
    }
}
