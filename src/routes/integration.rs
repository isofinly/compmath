use crate::compute::lab_three::{Function, IntegralCalculator, IntegrationMethod};
use graphul::{extract::Json, http::Methods, Context, Graphul};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::str;

use graphul::http::utils::header::CONTENT_TYPE;
use multipart::server::Multipart;
use regex::Regex;
use std::io::Read;

#[derive(Deserialize)]
struct IntegrationReqData {
    function_id: u8,
    method_id: u8,
    error: f64,
    lower_bound: f64,
    upper_bound: f64,
}

async fn integrate_from_string(ctx: Context) -> Json<Value> {
    let str_ref = ctx.body();

    if str_ref.is_empty() {
        return Json(json!({ "error": "Empty string" }));
    }

    let req_data: IntegrationReqData = match serde_json::from_str::<IntegrationReqData>(&str_ref) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Json(json!({ "error": "Failed to parse JSON" }));
        }
    };

    let function = match req_data.function_id {
        0 => Function::Polynomial,
        1 => Function::Sinus,
        2 => Function::Linear,
        3 => Function::Exponential,
        _ => return Json(json!({ "error": "Invalid function id" })),
    };

    let method = match req_data.method_id {
        0 => IntegrationMethod::LeftRectangles,
        1 => IntegrationMethod::RightRectangles,
        2 => IntegrationMethod::MiddleRectangles,
        3 => IntegrationMethod::Trapezoid,
        4 => IntegrationMethod::Simpson,
        _ => return Json(json!({ "error": "Invalid method id" })),
    };

    if req_data.error <= 0.0 {
        return Json(json!({ "error": "Error must be positive" }));
    }

    let calculator = IntegralCalculator::new(
        function,
        method,
        req_data.error,
        req_data.lower_bound,
        req_data.upper_bound,
    );

    let result = calculator.calculate_integral();

    Json(result)
}

async fn integrate_from_file(ctx: Context) -> Json<serde_json::Value> {
    let str_ref = ctx.body().as_str().to_string();
    let boundary = ctx
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok());

    // Other parts of your function remain unchanged
    let boundary = match boundary.and_then(|ct| Regex::new(r"boundary=(.*)").unwrap().captures(ct))
    {
        Some(captures) => captures.get(1).unwrap().as_str(),
        None => {
            return Json(json!({ "error": "Missing or invalid boundary in Content-Type header" }));
        }
    };

    let mut mp = Multipart::with_body(str_ref.as_bytes(), boundary);
    let mut buffer: Vec<u8> = Vec::new();

    // Corrected approach for reading multipart data
    while let Ok(Some(mut field)) = mp.read_entry() {
        let mut data = String::new();
        // Read the data into a string buffer
        if field.data.read_to_string(&mut data).is_ok() {
            // Assuming you want to process the data as a string, otherwise, adjust accordingly
            buffer.extend_from_slice(data.as_bytes());
        }
    }

    let req_data: IntegrationReqData = match serde_json::from_slice(&buffer) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Json(json!({ "error": "Failed to parse JSON" }));
        }
    };

    let function = match req_data.function_id {
        0 => Function::Polynomial,
        1 => Function::Sinus,
        2 => Function::Linear,
        3 => Function::Exponential,
        _ => return Json(json!({ "error": "Invalid function id" })),
    };

    let method = match req_data.method_id {
        0 => IntegrationMethod::LeftRectangles,
        1 => IntegrationMethod::RightRectangles,
        2 => IntegrationMethod::MiddleRectangles,
        3 => IntegrationMethod::Trapezoid,
        4 => IntegrationMethod::Simpson,
        _ => return Json(json!({ "error": "Invalid method id" })),
    };

    if req_data.error <= 0.0 {
        return Json(json!({ "error": "Error must be positive" }));
    }

    let calculator = IntegralCalculator::new(
        function,
        method,
        req_data.error,
        req_data.lower_bound,
        req_data.upper_bound,
    );

    let result = calculator.calculate_integral();
    Json(result)
}
pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut integration_group = router.group("integration");

    integration_group.post("/string", integrate_from_string);
    integration_group.post("/file", integrate_from_file);

    router
}
