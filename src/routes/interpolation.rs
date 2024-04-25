use graphul::{extract::Json, http::Methods, Context, Graphul};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::f64::consts::PI;
use std::io::BufRead;
use std::str;

use graphul::http::utils::header::CONTENT_TYPE;
use multipart::server::Multipart;
use regex::Regex;

use crate::compute::lab_five::generate_function_values;
use crate::compute::lab_five::InterpolationCalculator;
use crate::compute::lab_five::InterpolationMethod;

#[derive(Deserialize, Debug)]
struct InterpolationReqData {
    x: Vec<f64>,
    y: Vec<f64>,
    method: usize,
    function: usize,
    point: f64,
    nodes_amount: i32,
    start: f64,
    end: f64,
}

#[derive(Serialize)]
struct ErrorMsg {
    error: String,
}

async fn interpolate_from_string(ctx: Context) -> Json<Value> {
    let str_ref = ctx.body(); // Assumes a method to get the request body as a string

    if str_ref.is_empty() {
        return Json(json!({ "error": "Empty request body" }));
    }

    let req_data: InterpolationReqData = match serde_json::from_str(&str_ref) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse input JSON: {}", e);
            return Json(json!({ "error": "Failed to parse input data" }));
        }
    };

    if req_data.x.len() != req_data.y.len() || req_data.x.is_empty() {
        return Json(
            json!({ "error": "Invalid data: X and Y arrays must be of the same length and not empty" }),
        );
    }

    if req_data.nodes_amount == 0 {
        return Json(json!({ "error": "Invalid nodes amount" }));
    }

    let method = match req_data.method {
        0 => InterpolationMethod::Lagrange,
        1 => InterpolationMethod::NewtonSeparated,
        2 => InterpolationMethod::NewtonFinite,
        _ => return Json(json!({ "error": "Invalid method id" })),
    };

    if req_data.nodes_amount <= -1 {
        println!("{:?}", req_data);
        let calculator = InterpolationCalculator::new(method, req_data.x, req_data.y);
        println!(
            "Interpolated value {}",
            calculator.interpolate()(req_data.point)
        );
        println!("Latex: {}", calculator.print_latex());

    } else {
        if req_data.start > req_data.end {
            return Json(json!({ "error": "Invalid interval" }));
        };

        let function = match req_data.function {
            0 => f64::sin,
            1 => f64::cos,
            2 => f64::tan,
            _ => return Json(json!({ "error": "Invalid function id" })),
        };
        let (x, y) = generate_function_values(
            function,
            req_data.start,
            req_data.end,
            req_data.nodes_amount.abs() as usize,
        );
        let calculator = InterpolationCalculator::new(method, x, y);
        println!(
            "Interpolated value {}",
            calculator.interpolate()(req_data.point)
        );
        println!("Latex: {}", calculator.print_latex());
    }

    Json(json!({
        "data": ""
    }))
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut interpolation_group = router.group("interpolation");

    interpolation_group.post("/string", interpolate_from_string);
    // interpolation_group.post("/file", approximate_from_file);

    router
}
