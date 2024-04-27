#[derive(Clone, Copy)]
pub enum Equations {
    Linear,
    Fraction,
    Trigonometric,
}

impl Equations {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        match self {
            Self::Linear => -2.0 * y + x.powi(2),
            Self::Fraction => x.powi(3)-2.0*y,
            Self::Trigonometric => y * x.cos(),
        }
    }
}

impl Equations {
    fn evaluate_solution(&self, x: f64, c: f64) -> f64 {
        match self {
            Self::Linear => (x.powi(2) - x) / 2.0 + 0.25 + c / (2.0 * x).exp(),
            Self::Fraction => x.powi(3)/2.0-((3.0*x.powi(2)+3.0*x)/4.0)-(3.0/8.0)+(c/(2.0*x).exp()),
            Self::Trigonometric => c * (x.sin()).exp(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum MethodType {
    Euler,
    ExtendedEuler,
    Milne,
}

pub struct DifferentialEquationCalculator {
    equation: Equations,
    method: MethodType,
    error: f64,
    x0: f64,
    y0: f64,
    xn: f64,
    h: f64,
}

impl DifferentialEquationCalculator {
    pub fn new(
        equation: &Equations,
        method: &MethodType,
        error: f64,
        x0: f64,
        y0: f64,
        xn: f64,
        h: f64,
    ) -> Self {
        Self {
            equation: *equation,
            method: *method,
            error,
            x0,
            y0,
            xn,
            h,
        }
    }

    fn find_constant_c(&self) -> f64 {
        let target_y = self.y0;  // This is y0 from the initial condition
        let x = self.x0;
        let mut best_c = 0.0;
        let mut min_error = f64::INFINITY;

        // Iterate over a range of possible C values
        for c in (-1000..=1000).map(|i| i as f64 * 0.01) {  // Adjust range and step as needed
            let y = self.equation.evaluate_solution(x, c);
            let error = (y - target_y).abs();

            if error < min_error {
                min_error = error;
                best_c = c;

                if error < self.error {  // Utilize the given tolerance to possibly exit early
                    break;
                }
            }
        }

        best_c
    }

    pub fn get_equation_for_c(&self) -> String {
        let c = self.find_constant_c();
        match self.equation {
            Equations::Linear => format!("\\frac{{x^{{2}} - x}}{{2}} + 0.25 + \\frac{{{}}}{{\\exp(2.0 * x)}}", c),
            Equations::Fraction => format!("\\frac{{x^3}}{{2.0}} - \\frac{{3x^2 + 3x}}{{4.0}} - \\frac{{3}}{{8.0}} + \\exp(\\frac{{{}}}{{\\left(2.0x\\right)}})", c),
            Equations::Trigonometric => format!("{} \\times \\exp(\\sin(x))", c),
        }
    }

    fn euler(&self) -> Vec<(f64, f64)> {
        let mut x = vec![self.x0];
        let mut y = vec![self.y0];
        let mut h = self.h;
        let n = ((self.xn - self.x0) / h).floor() as i32;

        loop {
            for _i in 1..=n {
                let last_y = *y.last().unwrap();
                let last_x = *x.last().unwrap();
                y.push(last_y + h * self.equation.evaluate(last_x, last_y));
                x.push(last_x + h);
            }

            let h_new = h / 2.0;
            let mut x1 = vec![self.x0];
            let mut y1 = vec![self.y0];
            let n1 = ((self.xn - self.x0) / h_new).floor() as i32;
            for _i in 1..=n1 {
                let last_y = *y1.last().unwrap();
                let last_x = *x1.last().unwrap();
                y1.push(last_y + h_new * self.equation.evaluate(last_x, last_y));
                x1.push(last_x + h_new);
            }

            if (y[1] - y1[1]).abs() <= self.error {
                break;
            }

            h = h_new;
            x = x1;
            y = y1;
        }

        x.into_iter().zip(y).collect()
    }

    fn extended_euler(&self) -> Vec<(f64, f64)> {
        let mut x = vec![self.x0];
        let mut y = vec![self.y0];
        let mut h = self.h;
        let n = ((self.xn - self.x0) / h).floor() as i32;

        loop {
            for _ in 1..=n {
                let last_y = *y.last().unwrap();
                let last_x = *x.last().unwrap();
                let k1 = self.equation.evaluate(last_x, last_y);
                let k2 = self.equation.evaluate(last_x + h, last_y + h * k1);
                y.push(last_y + h / 2.0 * (k1 + k2));
                x.push(last_x + h);
            }

            let h_new = h / 2.0;
            let mut x1 = vec![self.x0];
            let mut y1 = vec![self.y0];
            let n1 = ((self.xn - self.x0) / h_new).floor() as i32;
            for _ in 1..=n1 {
                let last_y = *y1.last().unwrap();
                let last_x = *x1.last().unwrap();
                let k1 = self.equation.evaluate(last_x, last_y);
                let k2 = self.equation.evaluate(last_x + h_new, last_y + h_new * k1);
                y1.push(last_y + h_new / 2.0 * (k1 + k2));
                x1.push(last_x + h_new);
            }

            if ((y[1] - y1[1]).abs() / 3.0) <= self.error {
                break;
            }

            h = h_new;
            x = x1;
            y = y1;
        }

        x.into_iter().zip(y).collect()
    }

    fn milne(&self) -> Vec<(f64, f64)> {
        let mut x = vec![self.x0];
        let mut y = vec![self.y0];
        let n = ((self.xn - self.x0) / self.h).floor() as i32;

        for i in 1..=n {
            x.push(x[(i - 1) as usize] + self.h);
        }

        // Starting integration using simple method
        for i in 1..=std::cmp::min(n, 4) {
            let last_y = *y.last().unwrap();
            let last_x = *x.last().unwrap();
            let prev_x = x[(i - 1) as usize];
            let prev_y = y[(i - 1) as usize];
            let k1 = self.equation.evaluate(prev_x, prev_y);
            let k2 = self.equation.evaluate(last_x, last_y + self.h * k1);
            y.push(last_y + self.h / 2.0 * (k1 + k2));
        }

        // Applying Milne's method
        for i in 4..=n {
            let mut y_pred = y[(i - 4) as usize]
                + 4.0 * self.h / 3.0
                    * (2.0
                        * self
                            .equation
                            .evaluate(x[(i - 3) as usize], y[(i - 3) as usize])
                        - self
                            .equation
                            .evaluate(x[(i - 2) as usize], y[(i - 2) as usize])
                        + 2.0
                            * self
                                .equation
                                .evaluate(x[(i - 1) as usize], y[(i - 1) as usize]));
            let mut y_corr = y[(i - 2) as usize]
                + self.h / 3.0
                    * (self
                        .equation
                        .evaluate(x[(i - 2) as usize], y[(i - 2) as usize])
                        + 4.0
                            * self
                                .equation
                                .evaluate(x[(i - 1) as usize], y[(i - 1) as usize])
                        + self.equation.evaluate(x[i as usize], y_pred));

            while (y_pred - y_corr).abs() > self.error {
                y_pred = y_corr;
                y_corr = y[(i - 2) as usize]
                    + self.h / 3.0
                        * (self
                            .equation
                            .evaluate(x[(i - 2) as usize], y[(i - 2) as usize])
                            + 4.0
                                * self
                                    .equation
                                    .evaluate(x[(i - 1) as usize], y[(i - 1) as usize])
                            + self.equation.evaluate(x[i as usize], y_pred));
            }
            y.push(y_corr);
        }

        x.into_iter().zip(y).collect()
    }

    // Helper function to select method and solve
    pub fn solve(&self) -> Vec<(f64, f64)> {
        match self.method {
            MethodType::Euler => self.euler(),
            MethodType::ExtendedEuler => self.extended_euler(),
            MethodType::Milne => self.milne(),
        }
    }
}
