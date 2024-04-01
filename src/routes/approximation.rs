use graphul::{extract::Json, http::Methods, Context, Graphul};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::io::BufRead;
use std::io::Read;
use std::str;

use graphul::http::utils::header::CONTENT_TYPE;
use multipart::server::Multipart;
use regex::Regex;

use crate::compute::lab_four::find_best_function;
use crate::compute::lab_four::ApproximationCalculator;

#[derive(Deserialize, Debug)]
struct ApproximationReqData {
    x: Vec<f64>,
    y: Vec<f64>,
}

async fn approximate_from_string(ctx: Context) -> Json<Value> {
    let str_ref = ctx.body(); // Assumes a method to get the request body as a string

    if str_ref.is_empty() {
        return Json(json!({ "error": "Empty request body" }));
    }

    let req_data: ApproximationReqData = match serde_json::from_str(&str_ref) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Json(json!({ "error": "Failed to parse JSON" }));
        }
    };

    if req_data.x.len() != req_data.y.len() || req_data.x.is_empty() {
        return Json(
            json!({ "error": "Invalid data: X and Y arrays must be of the same length and not empty" }),
        );
    }

    let n = req_data.x.len();
    let function = find_best_function(n, &req_data.x, &req_data.y);
    let mut calculator = ApproximationCalculator::new(function, req_data.x, req_data.y);
    let coefficients = calculator.calculate_coefficients();
    let phi_values = calculator.get_phi_values();
    let epsilon_values = calculator.get_epsilon_values();
    let pearson_coefficient = match calculator.calculate_pearson_correlation() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to calculate Pearson correlation: {}", e);
            return Json(json!({ "error": "Failed to calculate Pearson correlation" }));
        }
    };

    Json(json!({
        "result": {
            "coefficients": coefficients,
            "function": calculator.print_function(),
            "standard_deviation": calculator.calculate_standard_deviation(),
            "pearson_correlation": pearson_coefficient,
            "differences": calculator.calculate_differences(),
            "phi_values": phi_values,
            "epsilon_values": epsilon_values,
            "data_points": calculator.x.iter().zip(calculator.y.iter()).zip(phi_values.iter()).zip(epsilon_values.iter())
            .map(|(((x, y), phi), eps)|
                json!({ "x": x, "y": y, "phi_x": phi, "epsilon": eps })
            )
            .collect::<Vec<_>>()
        }
    }))
}

async fn approximate_from_file(ctx: Context) -> Json<serde_json::Value> {
    let str_ref = ctx.body().as_str().to_string();
    let boundary = ctx.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();
    // panic!("{:?} \n \n {:?}", str_ref, boundary);

    let re: Regex = Regex::new(r"boundary=(.*)").unwrap();
    let captures = re.captures(boundary).unwrap();
    let boundary = captures.get(1).unwrap().as_str();

    let mut mp = Multipart::with_body(str_ref.as_bytes(), boundary);

    let mut buffer: Vec<u8> = Vec::new();
    

    while let Some(mut field) = mp.read_entry().unwrap() {
        let data = field.data.fill_buf().unwrap();
        buffer.extend_from_slice(data);
    }
    let str_ref = str::from_utf8(&buffer).unwrap();

    // panic!("{:?}", str_ref);
    let lines: Vec<&str> = str_ref.trim().splitn(2, '\n').collect();
    
    if lines.len() != 2 {
        return Json(json!({ "error": "The file must have two lines of numbers, separated by spaces." }));
    }
    
    // Parse the x and y values from the respective lines.
    let x_values: Vec<f64> = match lines[0].split_whitespace().map(str::parse).collect() {
        Ok(vals) => vals,
        Err(_) => return Json(json!({ "error": "Invalid x values." }))
    };
    
    let y_values: Vec<f64> = match lines[1].split_whitespace().map(str::parse).collect() {
        Ok(vals) => vals,
        Err(_) => return Json(json!({ "error": "Invalid y values." }))
    };
    let mut req_data = ApproximationReqData { x: x_values.clone(), y: y_values.clone() };
    req_data.x = x_values;
    req_data.y = y_values;
    let n = req_data.x.len();
    let function = find_best_function(n, &req_data.x, &req_data.y);
    let mut calculator = ApproximationCalculator::new(function, req_data.x, req_data.y);
    calculator.calculate_coefficients();

    let phi_values = calculator.get_phi_values();
    let epsilon_values = calculator.get_epsilon_values();

    let pearson_coefficient = match calculator.calculate_pearson_correlation() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to calculate Pearson correlation: {}", e);
            return Json(json!({ "error": "Failed to calculate Pearson correlation" }));
        }
    };

    Json(json!({
        "result": {
            "coefficients": calculator.coefficients,
            "function": calculator.print_function(),
            "standard_deviation": calculator.calculate_standard_deviation(),
            "pearson_correlation": pearson_coefficient,
            "differences": calculator.calculate_differences(),
            "phi_values": phi_values,
            "epsilon_values": epsilon_values,
            "data_points": calculator.x.iter().zip(calculator.y.iter()).zip(phi_values.iter()).zip(epsilon_values.iter())
            .map(|(((x, y), phi), eps)|
                json!({ "x": x, "y": y, "phi_x": phi, "epsilon": eps })
            )
            .collect::<Vec<_>>()
        }
    }))
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut approximation_group = router.group("approximation");

    approximation_group.post("/string", approximate_from_string);
    approximation_group.post("/file", approximate_from_file);

    router
}
