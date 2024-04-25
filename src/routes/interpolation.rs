use graphul::{extract::Json, http::Methods, Context, Graphul};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::io::BufRead;
use std::str;

use graphul::http::utils::header::CONTENT_TYPE;
use multipart::server::Multipart;
use regex::Regex;

use crate::compute::lab_five::InterpolationCalculator;
use crate::compute::lab_five::InterpolationMethod;


#[derive(Deserialize)]
struct InterpolationReqData {
    x: Vec<f64>,
    y: Vec<f64>,
    method: usize,
    function: usize
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

    let method = match req_data.method {
        0 => InterpolationMethod::Lagrange,
        1 => InterpolationMethod::NewtonSeparated,
        2 => InterpolationMethod::NewtonFinite,
        _ => return Json(json!({ "error": "Invalid method id" })),
        
    };


    let calculator = InterpolationCalculator::new(method, req_data.x, req_data.y);
    calculator.interpolate(0.0);
    calculator.get_interpolated_function()
        .iter()
        .for_each(|f| println!("function {:?}", f));
        println!("values {:?}",calculator.get_interpolated_values(0.0));
        println!("nodes {:?}",calculator.get_interpolation_nodes());
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
