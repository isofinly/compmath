use graphul::{http::Methods, Context, Graphul};
use tokio::fs;
use tracing::debug;

use crate::compute::Matrix;


async fn calculate_from_string(ctx: Context) -> String {
    let str_ref = ctx.body();
    let mut matrix = Matrix::new();
    matrix.init(&str_ref);
    let _ = fs::write("/home/isofinly/itmo/labs/compmath/testlab/matrix.txt", &str_ref).await;
    str_ref
}

async fn calculate_from_file(ctx: Context) -> String {
    let str_ref = ctx.body().as_str().to_string();
    debug!("file: {}", str_ref);
    str_ref
}


pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    let mut lin_eq_group = router.group("linear_equation");

    lin_eq_group.post("/string", calculate_from_string);
    lin_eq_group.post("/file", calculate_from_file);

    router
}