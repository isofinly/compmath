mod compute;
mod routes;

use graphul::{
    middleware::tower::cors::{self, Any},
    Graphul,
};
use routes::routes;

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.add_router(routes().await);

    app.middleware(
        cors::CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(Any)
            .allow_headers(Any),
    );

    // println!("{:?}", app.routes());

    app.run("127.0.0.1:8000").await;
}

