use graphul::{
    extract::Json,
    http::{utils::header::CONTENT_TYPE, Methods},
    Context, Graphul,
};
use multipart::server::Multipart;
use regex::Regex;
use serde::Deserialize;
use std::io::BufRead;
use std::panic;
use std::str;

use crate::compute::lab_two::{Equation, MethodType, NewtonSystemMethod, Solver, SystemEquations};

#[derive(Debug, Deserialize)]
struct EquationReqData {
    eq_id: usize,
    interval: [f64; 2],
    estimate: f64,
    method_id: usize,
}

async fn calculate_equation_from_string(ctx: Context) -> Json<serde_json::Value> {
    let str_ref = ctx.body();
    let req_id;
    let interval;
    let estimate;
    let method_id;

    if str_ref.is_empty() {
        return Json(serde_json::json!({ "error": "Empty string" }));
    }

    match serde_json::from_str::<EquationReqData>(&str_ref) {
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

    if !(0..4).contains(&req_id) {
        return Json(serde_json::json!({ "error": "Invalid equation id" }));
    }

    if !(0..4).contains(&method_id) {
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
        3 => Solver::new(&equation, MethodType::Secant),
        _ => panic!("Invalid method id"),
    };

    method.solve(interval[0], interval[1], estimate)
}

async fn calculate_equation_from_file(ctx: Context) -> Json<serde_json::Value> {
    let req_id;
    let interval;
    let estimate;
    let method_id;

    let str_ref = ctx.body().as_str().to_string();
    let boundary = ctx.headers().get(CONTENT_TYPE);

    let bndry = match boundary {
        Some(boundary) => boundary.to_str().unwrap(),
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

    match serde_json::from_str::<EquationReqData>(str::from_utf8(&buffer).unwrap()) {
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

    if !(0..4).contains(&req_id) {
        return Json(serde_json::json!({ "error": "Invalid equation id" }));
    }

    if !(0..4).contains(&method_id) {
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
        3 => Solver::new(&equation, MethodType::Secant),
        _ => return Json(serde_json::json!({ "error": "Invalid method id" })),
    };

    method.solve(interval[0], interval[1], estimate)
}

#[derive(Debug, Deserialize)]
struct SystemEquationsReqData {
    eq_id: usize,
    interval: [f64; 2],
    estimate: f64,
}

async fn calculate_system_from_string(ctx: Context) -> Json<serde_json::Value> {
    let str_ref = ctx.body();
    let req_id;
    let x0;
    let y0;
    let tolerance;

    if str_ref.is_empty() {
        return Json(serde_json::json!({ "error": "Empty string" }));
    }

    match serde_json::from_str::<EquationReqData>(&str_ref) {
        Ok(data) => {
            req_id = data.eq_id;
            x0 = data.interval[0];
            y0 = data.interval[1];
            tolerance = data.estimate;
        }
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Json(serde_json::json!({ "error": "Failed to parse JSON" }));
        }
    }
    if !(0..4).contains(&req_id) {
        return Json(serde_json::json!({ "error": "Invalid system of equations id" }));
    }

    let equations = SystemEquations::new(req_id.try_into().unwrap());

    let mut method = NewtonSystemMethod::new(x0, y0, tolerance, &equations);

    let result = method.solve();

    Json(result)
}

async fn calculate_system_from_file(ctx: Context) -> Json<serde_json::Value> {
    let x0;
    let y0;
    let tolerance;
    let req_id;

    let str_ref = ctx.body().as_str().to_string();
    let boundary = ctx.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();
    let re: Regex = Regex::new(r"boundary=(.*)").unwrap();
    let captures = re.captures(boundary).unwrap();
    let boundary = captures.get(1).unwrap().as_str();

    let mut mp = Multipart::with_body(str_ref.as_bytes(), boundary);

    let mut buffer: Vec<u8> = Vec::new();

    while let Some(mut field) = mp.read_entry().unwrap() {
        let data = field.data.fill_buf().unwrap();
        buffer.extend_from_slice(data);
    }

    match serde_json::from_slice::<SystemEquationsReqData>(&buffer) {
        Ok(data) => {
            x0 = data.interval[0];
            y0 = data.interval[1];
            tolerance = data.estimate;
            req_id = data.eq_id;
        }
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Json(serde_json::json!({ "error": "Failed to parse JSON" }));
        }
    }

    // Here, we assume you have a way to solve the system of equations similar to how individual equations are solved
    // For example:
    let equations = SystemEquations::new(req_id.try_into().unwrap());
    let mut method = NewtonSystemMethod::new(x0, y0, tolerance, &equations);
    let result = method.solve();

    Json(result)
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut non_lin_eq_group = router.group("nonlinear_equations");

    non_lin_eq_group.post("/string", calculate_equation_from_string);
    non_lin_eq_group.post("/file", calculate_equation_from_file);

    let mut non_lin_eqs_group = router.group("system_nonlinear_equations");

    non_lin_eqs_group.post("/string", calculate_system_from_string);
    non_lin_eqs_group.post("/file", calculate_system_from_file);

    router
}
