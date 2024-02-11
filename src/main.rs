mod routes;
mod compute;

use graphul::{Graphul,FolderConfig, FileConfig};
use routes::routes;

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();
    
    app.add_router(routes().await);

    // println!("{:?}", app.routes());

    app.static_file("/main", "frontend/index.html", FileConfig::default());


    app.run("127.0.0.1:8000").await;
}