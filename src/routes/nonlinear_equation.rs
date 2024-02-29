use std::io::BufRead;
use std::str;
use std::panic;
use graphul::{
    extract::Json,
    http::{utils::header::CONTENT_TYPE, Methods},
    Context, Graphul,
};
use multipart::server::Multipart;
use regex::Regex;
use serde::Deserialize;

use crate::compute::lab_two::{Equation, MethodType, Solver};

#[derive(Debug, Deserialize)]
struct ReqData {
    eq_id: usize,
    interval: [f64; 2],
    estimate: f64,
    method_id: usize,
}

async fn calculate_from_string(ctx: Context) -> Json<serde_json::Value> {
    let str_ref = ctx.body();
    let req_id;
    let interval;
    let estimate;
    let method_id;

    if str_ref.is_empty() {
        return Json(serde_json::json!({ "error": "Empty string" }));
    }

    match serde_json::from_str::<ReqData>(&str_ref) {
        Ok(data) => {
            req_id = data.eq_id;
            interval = data.interval;
            estimate = data.estimate;
            method_id = data.method_id;
        }
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Json(serde_json::json!({ "error": "Failed to parse JSON" }));
        }
    }

    if !(0..3).contains(&req_id) {
        return Json(serde_json::json!({ "error": "Invalid equation id" }));
    }

    if !(0..2).contains(&method_id) {
        return Json(serde_json::json!({ "error": "Invalid method id" }));
    }

    if estimate <= 0.0 {
        return Json(serde_json::json!({ "error": "Estimate must be positive" }));
    }

    let equation = Equation::new(req_id.try_into().unwrap());

    let mut method = match method_id {
        0 => Solver::new(&equation, MethodType::HalfDivision),
        1 => Solver::new(&equation, MethodType::Iteration),
        2 => Solver::new(&equation, MethodType::Newton),
        _ => panic!("Invalid method id"),
    };

    let result = method.solve(interval[0], interval[1], estimate);

    result
}

async fn calculate_from_file(ctx: Context) -> Json<serde_json::Value> {
    let req_id;
    let interval;
    let estimate;
    let method_id;
    let bndry;

    let str_ref = ctx.body().as_str().to_string();
    let boundary = ctx.headers().get(CONTENT_TYPE);

    match boundary {
        Some(boundary) => bndry = boundary.to_str().unwrap(),
        None => {
            return Json(serde_json::json!({ "error": "Missing boundary in Content-Type header" }));
        }
    };

    let re: Regex = Regex::new(r"boundary=(.*)").unwrap();

    let captures = re.captures(bndry).unwrap();
    let boundary = captures.get(1).unwrap().as_str();

    let mut mp = Multipart::with_body(str_ref.as_bytes(), boundary);

    let mut buffer: Vec<u8> = Vec::new();

    while let Some(mut field) = mp.read_entry().unwrap() {
        let data = field.data.fill_buf().unwrap();
        buffer.extend_from_slice(data);
    }

    match serde_json::from_str::<ReqData>(str::from_utf8(&buffer).unwrap()) {
        Ok(data) => {
            req_id = data.eq_id;
            interval = data.interval;
            estimate = data.estimate;
            method_id = data.method_id;
        }
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Json(serde_json::json!({ "error": "Failed to parse JSON" }));
        }
    }

    if !(0..3).contains(&req_id) {
        return Json(serde_json::json!({ "error": "Invalid equation id" }));
    }

    if !(0..2).contains(&method_id) {
        return Json(serde_json::json!({ "error": "Invalid method id" }));
    }

    if estimate <= 0.0 {
        return Json(serde_json::json!({ "error": "Estimate must be positive" }));
    }

    let equation = Equation::new(req_id.try_into().unwrap());

    let mut method = match method_id {
        0 => Solver::new(&equation, MethodType::HalfDivision),
        1 => Solver::new(&equation, MethodType::Iteration),
        2 => Solver::new(&equation, MethodType::Newton),
        _ => return Json(serde_json::json!({ "error": "Invalid method id" })),
    };

    let result = method.solve(interval[0], interval[1], estimate);

    result
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut lin_eq_group = router.group("nonlinear_equations");

    lin_eq_group.post("/string", calculate_from_string);
    lin_eq_group.post("/file", calculate_from_file);

    router
}
