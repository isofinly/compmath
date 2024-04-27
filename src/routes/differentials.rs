use graphul::{extract::Json, http::Methods, Context, Graphul};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::compute::lab_six::{DifferentialEquationCalculator, Equations, MethodType};

#[derive(Deserialize, Debug)]
struct DifferentialReqData {
    equation_id: usize,
    y0: f64,
    start: f64,
    end: f64,
    h: f64,
    error: f64,
}

async fn differential(ctx: Context) -> Json<Value> {
    let str_ref = ctx.body();

    if str_ref.is_empty() {
        return Json(json!({ "error": "Empty string" }));
    }

    let req_data: DifferentialReqData = match serde_json::from_str::<DifferentialReqData>(&str_ref)
    {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Json(json!({ "error": "Failed to parse JSON" }));
        }
    };

    let equation = match req_data.equation_id {
        0 => &Equations::Linear,
        1 => &Equations::Fraction,
        2 => &Equations::Trigonometric,
        _ => return Json(json!({ "error": "Invalid equation id" })),
    };

    if req_data.error <= 0.0 {
        return Json(json!({ "error": "Error must be positive" }));
    }

    if req_data.h <= 0.0 {
        return Json(json!({ "error": "Step must be positive" }));
    }

    if req_data.start >= req_data.end {
        return Json(json!({ "error": "Invalid interval" }));
    }

    let mut result = serde_json::Map::new();

    for method in &[
        MethodType::Euler,
        MethodType::ExtendedEuler,
        MethodType::Milne,
    ] {
        let method_name = match method {
            MethodType::Euler => "Euler",
            MethodType::ExtendedEuler => "ExtendedEuler",
            MethodType::Milne => "Milne",
        };

        let calculator = DifferentialEquationCalculator::new(
            equation,
            method,
            req_data.error,
            req_data.start,
            req_data.y0,
            req_data.end,
            req_data.h,
        );
        let partial_result = calculator.solve();

        result.insert(
            method_name.to_string(),
            json!({"points":partial_result,
                "equation": calculator.get_equation_for_c(), 
            }),
        );
    }
    Json(json!({ "result": result }))
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut differentials_group = router.group("differentials");

    differentials_group.post("/string", differential);

    router
}
