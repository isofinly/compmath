mod about;
mod linear_equation;
mod nonlinear_equations;
mod integration;
mod approximation;
mod interpolation; 
mod differentials;

use graphul::Graphul;

pub async fn routes() -> Graphul {
    let mut router = Graphul::router();

    router.add_routers(vec![
        about::routes().await,
        linear_equation::routes().await,
        nonlinear_equations::routes().await,
        integration::routes().await,
        approximation::routes().await,
        interpolation::routes().await,
        differentials::routes().await
    ]);

    router
}