use graphul::{extract::Json, http::Methods, Context, Graphul};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::str;



use crate::compute::lab_five::generate_function_values;
use crate::compute::lab_five::InterpolationCalculator;
use crate::compute::lab_five::InterpolationMethod;

#[derive(Deserialize, Debug)]
struct InterpolationReqData {
    x: Vec<f64>,
    y: Vec<f64>,
    function: usize,
    point: f64,
    nodes_amount: i32,
    start: f64,
    end: f64,
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

    if req_data.x.len() != req_data.y.len() {
        return Json(
            json!({ "error": "Invalid data: X and Y arrays must be of the same length and not empty" }),
        );
    }

    if req_data.nodes_amount == 0 {
        return Json(json!({ "error": "Invalid nodes amount" }));
    }

    if req_data.point < req_data.start || req_data.point > req_data.end {
        return Json(json!({ "error": "Invalid point" }));
    }

    let mut result = serde_json::Map::new();

    for method in &[
        InterpolationMethod::Lagrange,
        InterpolationMethod::NewtonSeparated,
        InterpolationMethod::NewtonFinite,
        InterpolationMethod::Stirling,
        InterpolationMethod::Bessel,
    ] {
        if req_data.nodes_amount <= -1 {
            let calculator = InterpolationCalculator::new(*method, req_data.x.clone(), req_data.y.clone());

            let method_name = match method {
                InterpolationMethod::Lagrange => "lagrange",
                InterpolationMethod::NewtonSeparated => "newton_separated",
                InterpolationMethod::NewtonFinite => "newton_finite",
                InterpolationMethod::Stirling => "stirling",
                InterpolationMethod::Bessel => "bessel",
            };

            if method_name != "lagrange" {
                result.insert(
                    method_name.to_string(),
                    json!({
                        "interpolated_value": calculator.interpolate()(req_data.point),
                        "latex_function": calculator.print_latex(),
                        "difference_table": calculator.difference_table(),
                        "nodes": calculator.get_nodes()
                    }),
                );
            } else {
                result.insert(
                    method_name.to_string(),
                    json!({
                        "interpolated_value": calculator.interpolate()(req_data.point),
                        "latex_function": calculator.print_latex(),
                        "nodes": calculator.get_nodes()
                    }),
                );
            }
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
                req_data.nodes_amount.unsigned_abs() as usize,
            );

            let calculator = InterpolationCalculator::new(*method, x, y);

            let method_name = match method {
                InterpolationMethod::Lagrange => "lagrange",
                InterpolationMethod::NewtonSeparated => "newton_separated",
                InterpolationMethod::NewtonFinite => "newton_finite",
                InterpolationMethod::Stirling => "stirling",
                InterpolationMethod::Bessel => "bessel",
            };

            result.insert(
                method_name.to_string(),
                json!({
                    "interpolated_value": calculator.interpolate()(req_data.point),
                    "latex_function": calculator.print_latex(),
                    "difference_table": calculator.difference_table(),
                    "nodes": calculator.get_nodes()
                }),
            );
        }
    }

    Json(json!({ "result": result }))
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut interpolation_group = router.group("interpolation");

    interpolation_group.post("/string", interpolate_from_string);

    router
}
