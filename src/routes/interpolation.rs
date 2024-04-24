use graphul::{extract::Json, http::Methods, Context, Graphul};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::io::BufRead;
use std::str;

use graphul::http::utils::header::CONTENT_TYPE;
use multipart::server::Multipart;
use regex::Regex;


#[derive(Deserialize, Debug)]
struct InterpolationReqData {
    x: Vec<f64>,
    y: Vec<f64>,
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
