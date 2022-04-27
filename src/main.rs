use axum;

use axum_shuttle_demo;

const MERRIAM_WEBSTER_API_KEY: Option<&str> = option_env!("MERRIAM_WEBSTER_API_KEY");

#[tokio::main]
async fn main() {
    println!("Spinning server up on http://localhost:3000");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(axum_shuttle_demo::get_router(MERRIAM_WEBSTER_API_KEY).into_make_service())
        .await
        .unwrap();
}
