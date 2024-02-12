use std::io::BufRead;
use std::str;

use graphul::{
    extract::Json,
    http::{utils::header::CONTENT_TYPE, Methods},
    Context, Graphul,
};
use multipart::server::Multipart;
use regex::Regex;

use crate::compute::Matrix;

async fn calculate_from_string(ctx: Context) -> Json<serde_json::Value> {
    let str_ref = ctx.body();
    let mut matrix = Matrix::new();
    matrix.init(&str_ref);

    matrix.solve()
}

async fn calculate_from_file(ctx: Context) -> Json<serde_json::Value> {
    let str_ref = ctx.body().as_str().to_string();
    let boundary = ctx.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();

    let re = Regex::new(r"boundary=(.*)").unwrap();
    let captures = re.captures(boundary).unwrap();
    let boundary = captures.get(1).unwrap().as_str();

    let mut mp = Multipart::with_body(str_ref.as_bytes(), boundary);

    let mut buffer: Vec<u8> = Vec::new();

    while let Some(mut field) = mp.read_entry().unwrap() {
        let data = field.data.fill_buf().unwrap();
        buffer.extend_from_slice(data);
    }

    let mut matrix = Matrix::new();
    let str_ref = str::from_utf8(&buffer).unwrap();
    matrix.init_from_file(str_ref).unwrap();

    // String::from(str_ref);
    matrix.solve()
}

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut lin_eq_group = router.group("linear_equation");

    lin_eq_group.post("/string", calculate_from_string);
    lin_eq_group.post("/file", calculate_from_file);

    router
}
