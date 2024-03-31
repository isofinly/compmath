use graphul::{extract::Json, http::Methods, Context, Graphul};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::io::Read;
use std::str;

use graphul::http::utils::header::CONTENT_TYPE;
use multipart::server::Multipart;
use regex::Regex;

use crate::compute::lab_four::find_best_function;
use crate::compute::lab_four::ApproximationCalculator;

#[derive(Deserialize)]
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
        println!("str_ref: {}", str_ref);

    let boundary = ctx
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .and_then(|ct| Regex::new(r"boundary=(.*)").unwrap().captures(ct))
        .and_then(|captures| captures.get(1));

    let boundary = match boundary {
        Some(b) => b.as_str().trim(),
        None => {
            return Json(json!({ "error": "Missing or invalid boundary in Content-Type header" }))
        }
    };

    let mut mp = Multipart::with_body(str_ref.as_bytes(), boundary);
    let mut buffer: Vec<u8> = Vec::new();

    while let Ok(Some(mut field)) = mp.read_entry() {
        let mut data = String::new();
        if field.data.read_to_string(&mut data).is_ok() {
            buffer.extend_from_slice(data.as_bytes());
        }
    }

    let req_data: ApproximationReqData = match serde_json::from_slice(&buffer) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}",e);
            return Json(json!({ "error": "Failed to parse JSON" }));
        }
    };

    if req_data.x.len() != req_data.y.len() || req_data.x.is_empty() {
        return Json(json!({ "error": "X and Y arrays must be of the same length and not empty" }));
    }

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
