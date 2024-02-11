mod about;
mod linear_equation;

use graphul::Graphul;

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    router.add_routers(vec![
        about::routes().await,
        linear_equation::routes().await,
    ]);

    router
}