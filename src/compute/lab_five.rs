#[derive(Clone, Copy, Debug)]
pub enum InterpolationMethod {
    Lagrange,
    NewtonSeparated,
    NewtonFinite,
    Stirling,
    Bessel,
}

pub struct InterpolationCalculator {
    method: InterpolationMethod,
    x: Vec<f64>,
    y: Vec<f64>,
}

impl InterpolationCalculator {
    pub fn new(method: InterpolationMethod, x: Vec<f64>, y: Vec<f64>) -> Self {
        InterpolationCalculator { method, x, y }
    }

    pub fn get_nodes(&self) -> Vec<Vec<f64>> {
        vec![self.x.clone(), self.y.clone()]
    }

    pub fn interpolate<'a>(&'a self) -> Box<dyn Fn(f64) -> f64 + 'a> {
        match self.method {
            InterpolationMethod::Lagrange => self.lagrange(),
            InterpolationMethod::NewtonSeparated => self.newton_separated(),
            InterpolationMethod::NewtonFinite => self.newton_finite(),
            InterpolationMethod::Stirling => self.stirling(),
            InterpolationMethod::Bessel => self.bessel(),
        }
    }

    pub fn print_latex(&self) -> String {
        match self.method {
            InterpolationMethod::Lagrange => self.lagrange_latex(),
            InterpolationMethod::NewtonSeparated => self.newton_separated_latex(),
            InterpolationMethod::NewtonFinite => self.newton_finite_latex(),
            _ => String::from("None"),
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

    fn newton_separated(&self) -> Box<dyn Fn(f64) -> f64> {
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
                diff[j] = (diff[j] - diff[j - 1]) / (self.x[j] - self.x[j - i]);
            }
        }
        diff
    }

    fn newton_finite<'a>(&'a self) -> Box<dyn Fn(f64) -> f64 + 'a> {
        let n = self.x.len() - 1;
        let defy = self.difference_table();

        Box::new(move |v| {
            let mut result = defy[0][0];
            let mut term = 1.0;

            for i in 1..=n {
                term *= v - self.x[i - 1];
                result += term * defy[0][i] / factorial(i);
            }

            result
        })
    }

    fn stirling<'a>(&'a self) -> Box<dyn Fn(f64) -> f64 + 'a> {
        let n = self.x.len() - 1;
        let center = n / 2;
        let a = self.x[center];
        let defy = self.difference_table();

        Box::new(move |v| {
            let h = self.x[center + 1] - self.x[center];
            let t = (v - a) / h;

            let mut result = defy[center][0]
                + t * (defy[center - 1][1] + defy[center][1]) / 2.0
                + t * t / 2.0 * defy[center - 1][2];
            let mut term = t * t / 2.0;

            for k in 3..n {
                if k % 2 == 0 {
                    term *= t / k as f64;
                    result += term * defy[center - k / 2][k];
                } else {
                    term *= (t * t - (k / 2) as f64 * (k / 2) as f64) / (k as f64 * t);
                    result += term * (defy[center - k / 2 - 1][k] + defy[center - k / 2][k]) / 2.0;
                }
            }
            result
        })
    }

    fn bessel<'a>(&'a self) -> Box<dyn Fn(f64) -> f64 + 'a> {
        let n = self.x.len() - 1;
        let center = n / 2;
        let a = self.x[center];
        let defy = self.difference_table();

        Box::new(move |v| {
            let h = self.x[center + 1] - self.x[center];
            let t = (v - a) / h;
            let mut result = (defy[center][0] + defy[center + 1][0]) / 2.0
                + (t - 0.5) * defy[center][1]
                + t * (t - 1.0) / 2.0 * (defy[center - 1][2] + defy[center][2]) / 2.0;

            let mut term = t * (t - 1.0) / 2.0;

            for k in 3..(n + 1) {
                if k % 2 == 0 {
                    term /= t - 0.5;
                    term *= (t + ((k / 2 - 1) as f64)) * (t - (k / 2) as f64) / k as f64;
                    result += term
                        * (defy[center - 1 - (k / 2 - 1)][k] + defy[center - (k / 2 - 1)][k])
                        / 2.0;
                } else {
                    term *= (t - 0.5) / k as f64;
                    result += term * defy[center - k / 2][k];
                }
            }

            result
        })
    }

    pub fn difference_table(&self) -> Vec<Vec<f64>> {
        let n = self.y.len();
        let mut defy: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

        for i in 0..n {
            defy[i][0] = self.y[i];
        }

        for i in 1..n {
            for j in 0..(n - i) {
                defy[j][i] = defy[j + 1][i - 1] - defy[j][i - 1];
            }
        }
        // println!("{:?}", defy);

        defy
    }
}

impl InterpolationCalculator {
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
        terms.join(" + ").to_string()
    }

    fn newton_separated_latex(&self) -> String {
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
        terms.join(" + ").to_string()
    }

    fn newton_finite_latex(&self) -> String {
        let n = self.x.len() - 1;
        let defy = self.difference_table();

        let mut terms = vec![format!("{}", defy[0][0])];

        for i in 1..=n {
            let mut term_parts = vec![format!("(x - {})", self.x[0])];
            for j in 1..i {
                term_parts.push(format!("(x - {})", self.x[j]));
            }
            let term = format!(
                "+ \\frac{{{}}}{{{}!}} \\cdot {}",
                defy[0][i],
                i,
                term_parts.join(" \\cdot ")
            );
            terms.push(term);
        }

        terms.join(" ")
    }
}

fn factorial(n: usize) -> f64 {
    if n == 0 {
        1.0
    } else {
        n as f64 * factorial(n - 1)
    }
}

pub fn generate_function_values(
    func: fn(f64) -> f64,
    start: f64,
    end: f64,
    nodes_amount: usize,
) -> (Vec<f64>, Vec<f64>) {
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
