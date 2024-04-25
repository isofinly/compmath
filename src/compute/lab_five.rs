use std::fmt;

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
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

impl fmt::Display for InterpolationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl InterpolationCalculator {
    pub fn new(method: InterpolationMethod, x: Vec<f64>, y: Vec<f64>) -> Self {
        InterpolationCalculator { method, x, y }
    }

    pub fn from_function(method: InterpolationMethod, func: StandardFunction, start: f64, end: f64, num_points: usize) -> Result<Self, &'static str> {
        if num_points < 1 {
            return Err("Number of points must be greater than 1.");
        }
        if start >= end {
            return Err("Start must be less than end.");
        }

        let step = (end - start) / (num_points as f64 - 1.0);
        let mut x = Vec::with_capacity(num_points);
        let mut y = Vec::with_capacity(num_points);

        for i in 0..num_points {
            let x_value = start + step * i as f64;
            let y_value = match func {
                StandardFunction::Sin => x_value.sin(),
                StandardFunction::Cos => x_value.cos(),
                StandardFunction::Tan => x_value.tan(),
            };

            x.push(x_value);
            y.push(y_value);
        }

        Ok(InterpolationCalculator {
            method,
            x,
            y,
        })
    }

    pub fn interpolate(&self, x_query: f64) -> f64 {
        match self.method {
            InterpolationMethod::Lagrange => self.lagrange(x_query),
            InterpolationMethod::NewtonSeparated => self.newton_separated(x_query),
            InterpolationMethod::NewtonFinite => self.newton_finite(x_query),
        }
    }

    pub fn get_interpolated_values(&self, x_query: f64) -> Vec<f64> {
        vec![
            self.lagrange(x_query),
            self.newton_separated(x_query),
            self.newton_finite(x_query),
        ]
    }

    // Метод для получения узлов интерполяции
    pub fn get_interpolation_nodes(&self) -> (&Vec<f64>, &Vec<f64>) {
        (&self.x, &self.y)
    }

    fn lagrange(&self, x: f64) -> f64 {
        let mut result = 0.0;
        let n = self.x.len();
        for i in 0..n {
            let mut term = self.y[i];
            for j in 0..n {
                if i != j {
                    term *= (x - self.x[j]) / (self.x[i] - self.x[j]);
                }
            }
            result += term;
        }
        result
    }

    fn newton_separated(&self, x: f64) -> f64 {
        let n = self.x.len();
        let mut divided_diff = self.y.clone();
        for i in 1..n {
            for j in (i..n).rev() {
                divided_diff[j] = (divided_diff[j] - divided_diff[j-1]) / (self.x[j] - self.x[j-i]);
            }
        }

        let mut result = divided_diff[n-1];
        for i in (0..n-1).rev() {
            result = result * (x - self.x[i]) + divided_diff[i];
        }
        result
    }

    fn newton_finite(&self, x: f64) -> f64 {
        let n = self.x.len();
        let h = self.x[1] - self.x[0]; // Assuming evenly spaced x values for simplicity
        let mut finite_diff = self.y.clone();

        for i in 1..n {
            for j in (i..n).rev() {
                finite_diff[j] = finite_diff[j] - finite_diff[j-1];
            }
        }

        let mut result = finite_diff[n-1];
        let mut q = (x - self.x[n-1]) / h;
        for i in 1..n {
            result = result * q + finite_diff[n-1-i];
            q = (q * (x - self.x[n-1-i])) / (i as f64 + 1.0);
        }
        result
    }
     // Расчёт коэффициентов для многочлена Лагранжа
     fn lagrange_coefficients(&self) -> Vec<f64> {
        let n = self.x.len();
        let mut coeffs = vec![0.0; n];
        for i in 0..n {
            let mut li = 1.0;
            for j in 0..n {
                if i != j {
                    li *= self.x[i] - self.x[j];
                }
            }
            for j in 0..n {
                if i != j {
                    let term = self.y[i] / li;
                    for k in 0..n {
                        if k != i && k != j {
                            coeffs[k] += term / (self.x[i] - self.x[k]);
                        }
                    }
                    coeffs[j] -= term;
                }
            }
            coeffs[i] += self.y[i] / li;
        }
        coeffs
    }

    // Расчёт коэффициентов для многочлена Ньютона с разделенными разностями
    fn newton_separated_coefficients(&self) -> Vec<f64> {
        let n = self.x.len();
        let mut coeffs = self.y.clone();
        for i in 1..n {
            for j in (i..n).rev() {
                coeffs[j] = (coeffs[j] - coeffs[j-1]) / (self.x[j] - self.x[j-i]);
            }
        }
        coeffs.truncate(n); // Убираем лишние элементы
        coeffs
    }

    // Расчёт коэффициентов для многочлена Ньютона с конечными разностями
    fn newton_finite_coefficients(&self) -> Vec<f64> {
        let n = self.x.len();
        let mut coeffs = self.y.clone();
        let mut temp = self.y.clone();

        for i in 1..n {
            for j in 0..n-i {
                temp[j] = temp[j+1] - temp[j];
            }
            coeffs[i] = temp[0]; // Первый элемент каждого шага это i-ая конечная разность
        }
        coeffs.truncate(n); // Обрезаем коэффициенты до количества точек
        coeffs
    }

    // Функция для генерации LaTeX представления многочлена
    fn print_polynomial_latex(&self, coeffs: &[f64], x_values: &[f64]) -> String {
        let mut polynomial = coeffs[0].to_string();
        
        let mut cumulative_product = String::new();
        for (i, coeff) in coeffs.iter().enumerate().skip(1) {
            if *coeff != 0.0 {
                cumulative_product.push_str(&format!("(x - {:.3})", x_values[i - 1]));
                let sign = if *coeff < 0.0 { "-" } else { "+" };
                polynomial += &format!(" {} {:.3}{}", sign, coeff.abs(), cumulative_product);
            }
        }

        polynomial.replace("+ -", "- ")
    }

    // Получение LaTeX представлений функций интерполяции
    pub fn get_interpolated_function(&self) -> Vec<String> {
        vec![
            self.print_polynomial_latex(&self.lagrange_coefficients(), &self.x),
            self.print_polynomial_latex(&self.newton_separated_coefficients(), &self.x),
            self.print_polynomial_latex(&self.newton_finite_coefficients(), &self.x),
        ]
    }
}
